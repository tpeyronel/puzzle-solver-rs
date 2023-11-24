use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Arc},
    thread::{self, available_parallelism, JoinHandle},
    time::{Duration, Instant},
};

use serde::Serialize;
use tokio::sync::mpsc;

use super::{board::Board, common::Vec2, shape::Shape};

const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, Clone)]
struct Candidate {
    id: String,
    variations: Vec<Shape>,
}

#[derive(Debug, Clone)]
struct SolutionWithBorrows<'a> {
    placed_shapes: Vec<(Vec2, &'a Shape)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Solution {
    pub placed_shapes: Vec<(Vec2, Shape)>,
}

impl<'a> From<&SolutionWithBorrows<'a>> for Solution {
    fn from(solution: &SolutionWithBorrows<'a>) -> Self {
        Self {
            placed_shapes: solution.placed_shapes.iter().map(|&(p, s)| (p, s.clone())).collect(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const COL_WIDTH: usize = 8;

        let mut output = format!(
            "{0: ^w$}   {1: ^w$}   {2: ^w$}   {3: ^w$}\n",
            "shape",
            "rot",
            "pos",
            "flipped",
            w = COL_WIDTH,
        );

        for (p, s) in &self.placed_shapes {
            output += &format!(
                "{0: ^w$}   {1: ^w$}   {2: ^w$}   {3: ^w$}\n",
                format!("\"{}\"", s.metadata().id),
                s.metadata().rot,
                format!("({:>2},{:>2})", p.x, p.y),
                if s.metadata().flipped { "y" } else { "n" },
                w = COL_WIDTH,
            );
        }

        f.write_str(&output)
    }
}

pub enum SolverMessage {
    Ping,
    SolversBegin { threads: u32 },
    SolutionMessage(SolutionMessage),
    SolverEnd { elapsed: Duration },
}

pub struct SolutionMessage {
    pub threadi: u32,
    pub payload: SolutionPayload,
}

pub enum SolutionPayload {
    PartialSolution(Solution),
    TotalSolution(Solution),
}

struct SolveTask {
    width: u32,
    height: u32,
    sol_tx: mpsc::UnboundedSender<SolverMessage>,
    seed: Vec<(Vec2, Shape)>,
    candidates: Vec<Candidate>,
    remaining: Vec<u32>,
}

struct Solver<'a> {
    threadi: u32,
    board: Board,
    total_remaining: u32,
    solution: SolutionWithBorrows<'a>,
    sol_tx: mpsc::UnboundedSender<SolverMessage>,
    should_report: &'a AtomicBool,
}

impl<'a> Solver<'a> {
    fn new(
        threadi: u32,
        width: u32,
        height: u32,
        sol_tx: mpsc::UnboundedSender<SolverMessage>,
        should_report: &'a AtomicBool,
    ) -> Self {
        Self {
            threadi,
            board: Board::new(width, height),
            total_remaining: 0,
            solution: SolutionWithBorrows { placed_shapes: vec![] },
            sol_tx,
            should_report,
        }
    }

    fn solve(&mut self, candidates: &'a Vec<Candidate>, remaining: &mut Vec<u32>) {
        self.total_remaining = remaining.iter().sum();

        let start = Instant::now();
        let solved = self.solve_rec(candidates, remaining, Vec2::ZERO);
        let elapsed = start.elapsed();

        let _ = self.sol_tx.send(SolverMessage::SolverEnd { elapsed });

        if solved {
            let _ = self.sol_tx.send(SolverMessage::SolutionMessage(SolutionMessage {
                threadi: self.threadi,
                payload: SolutionPayload::TotalSolution((&self.solution).into()),
            }));
        }
    }

    fn try_seed(&mut self, seed: &'a Vec<(Vec2, Shape)>) -> bool {
        for (p, s) in seed {
            if !self.board.fits_at(s.mesh(), *p) {
                return false;
            }

            self.board.insert_at(s.mesh(), *p);
            self.solution.placed_shapes.push((*p, s));
        }

        return true;
    }

    fn solve_rec(&mut self, candidates: &'a Vec<Candidate>, remaining: &mut Vec<u32>, pos: Vec2) -> bool {
        if self.total_remaining == 0 {
            return true;
        }

        if self.should_report.load(std::sync::atomic::Ordering::Relaxed) {
            self.should_report.store(false, std::sync::atomic::Ordering::Relaxed);

            let message = SolverMessage::SolutionMessage(SolutionMessage {
                threadi: self.threadi,
                payload: SolutionPayload::PartialSolution((&self.solution).into()),
            });

            if self.sol_tx.send(message).is_err() {
                return true;
            }
        }

        if pos.x >= self.board.width() || pos.y >= self.board.height() {
            return false;
        }

        for (i, c) in candidates.iter().enumerate() {
            if remaining[i] == 0 {
                continue;
            }

            remaining[i] -= 1;
            self.total_remaining -= 1;

            for v in &c.variations {
                if self.board.fits_at(v.mesh(), pos) {
                    self.solution.placed_shapes.push((pos, v));
                    self.board.insert_at(v.mesh(), pos);

                    if self.solve_rec(candidates, remaining, pos) {
                        return true;
                    }

                    self.board.remove_at(v.mesh(), pos);
                    self.solution.placed_shapes.pop();
                }
            }

            self.total_remaining += 1;
            remaining[i] += 1;
        }

        let next_pos = if pos.y + 1 == self.board.height() {
            Vec2::new(pos.x + 1, 0)
        } else {
            Vec2::new(pos.x, pos.y + 1)
        };

        return self.solve_rec(candidates, remaining, next_pos);
    }
}

pub fn solve_async(
    width: u32,
    height: u32,
    shapes: Vec<Shape>,
) -> (
    mpsc::UnboundedReceiver<SolverMessage>,
    (Arc<AtomicBool>, Vec<JoinHandle<()>>),
) {
    let (task_tx, task_rx) = crossbeam_channel::unbounded::<SolveTask>();
    let workers = spawn_workers(&task_rx);

    let (sol_tx, sol_rx) = mpsc::unbounded_channel::<SolverMessage>();
    send_solve_tasks(width, height, shapes, sol_tx, task_tx);

    return (sol_rx, workers);
}

pub fn solve(width: u32, height: u32, shapes: Vec<Shape>) -> Option<Solution> {
    let (mut sol_rx, (terminate_timer, workers)) = solve_async(width, height, shapes);

    let mut sum = Duration::ZERO;
    let mut count = 0;

    while let Some(msg) = sol_rx.blocking_recv() {
        match msg {
            SolverMessage::SolutionMessage(SolutionMessage {
                payload: SolutionPayload::TotalSolution(sol),
                ..
            }) => {
                // Drop sol_rx so that workers finish early
                drop(sol_rx);

                // Wait for all workers to finish
                for w in workers {
                    let _ = w.join();
                }

                terminate_timer.store(true, std::sync::atomic::Ordering::Relaxed);
                return Some(sol);
            }
            SolverMessage::SolverEnd { elapsed } => {
                sum += elapsed;
                count += 1;
            }
            _ => continue,
        }
    }

    println!("Took: {} ms", (sum / count).as_millis());

    terminate_timer.store(true, std::sync::atomic::Ordering::Relaxed);
    None
}

fn spawn_workers(task_rx: &crossbeam_channel::Receiver<SolveTask>) -> (Arc<AtomicBool>, Vec<JoinHandle<()>>) {
    let count = available_parallelism().unwrap().get();

    let should_report_vars: Vec<Arc<AtomicBool>> = (0..count).map(|_| Arc::new(AtomicBool::new(false))).collect();

    let terminate_timer = Arc::new(AtomicBool::new(false));

    {
        let terminate_timer = terminate_timer.clone();
        let should_report_vars = should_report_vars.clone();
        thread::spawn(move || {
            while !terminate_timer.load(std::sync::atomic::Ordering::Relaxed) {
                should_report_vars
                    .iter()
                    .for_each(|sr| sr.store(true, std::sync::atomic::Ordering::Relaxed));

                thread::sleep(UPDATE_INTERVAL);
            }
        });
    }

    (
        terminate_timer,
        should_report_vars
            .into_iter()
            .enumerate()
            .map(|(i, should_report)| {
                let task_rx = task_rx.clone();

                thread::spawn(move || run_worker(i as u32, task_rx, should_report))
            })
            .collect(),
    )
}

fn run_worker(threadi: u32, task_rx: crossbeam_channel::Receiver<SolveTask>, should_report: Arc<AtomicBool>) {
    while let Ok(t) = task_rx.recv() {
        let SolveTask {
            width,
            height,
            sol_tx,
            seed,
            candidates,
            mut remaining,
        } = t;

        if sol_tx.send(SolverMessage::Ping).is_err() {
            break;
        }

        let mut solver = Solver::new(threadi, width, height, sol_tx, &should_report);

        if !solver.try_seed(&seed) {
            continue;
        }

        solver.solve(&candidates, &mut remaining);
    }
}

fn send_solve_tasks(
    width: u32,
    height: u32,
    shapes: Vec<Shape>,
    sol_tx: mpsc::UnboundedSender<SolverMessage>,
    task_tx: crossbeam_channel::Sender<SolveTask>,
) {
    let (candidates, remaining) = shapes_to_candidates(shapes);

    for x in 0..width {
        for y in 0..height {
            for (i, c) in candidates.iter().enumerate() {
                for v in &c.variations {
                    let sol_tx = sol_tx.clone();
                    let seed = vec![(Vec2 { x, y }, v.clone())];
                    let candidates = candidates.clone();
                    let mut remaining = remaining.clone();
                    remaining[i] -= 1;

                    let task = SolveTask {
                        width,
                        height,
                        sol_tx,
                        seed,
                        candidates,
                        remaining,
                    };

                    task_tx.send(task).unwrap();
                }
            }
        }
    }
}

fn shapes_to_candidates(shapes: Vec<Shape>) -> (Vec<Candidate>, Vec<u32>) {
    shapes
        .iter()
        .map(|s| {
            (
                Candidate {
                    id: s.metadata().id.clone(),
                    variations: compute_shape_variations(s),
                },
                1,
            )
        })
        .collect::<Vec<(Candidate, u32)>>()
        .into_iter()
        .unzip()
}

fn compute_shape_variations(shape: &Shape) -> Vec<Shape> {
    fn push_rotations(mut shape: Shape, variations: &mut Vec<Shape>) {
        for _ in 0..4 {
            let next = shape.rotated_ccw();
            variations.push(shape);
            shape = next;
        }
    }

    let mut variations = vec![];

    push_rotations(shape.clone(), &mut variations);
    push_rotations(shape.flipped_hor(), &mut variations);

    variations
}

#[cfg(test)]
mod tests {
    use crate::logic::{
        digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9},
        solver,
    };

    #[test]
    fn solve_works_for_digits_8() {
        let digits = vec![digit8()];

        let solution = solver::solve(2, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_3_8() {
        let digits = vec![digit3(), digit8()];

        let solution = solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_0_1_7() {
        let digits = vec![digit0(), digit1(), digit7()];

        let solution = solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_6_9() {
        let digits = vec![digit9(), digit6()];

        let solution = solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_3_6_9() {
        let digits = vec![digit3(), digit9(), digit6()];

        let solution = solver::solve(3, 4, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_0_1_4_7_8() {
        let digits = vec![digit0(), digit1(), digit4(), digit7(), digit8()];

        let solution = solver::solve(5, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_all_digits() {
        let digits = vec![
            digit0(),
            digit1(),
            digit2(),
            digit3(),
            digit4(),
            digit5(),
            digit6(),
            digit7(),
            digit8(),
            digit9(),
        ];

        let solution = solver::solve(6, 5, digits);

        println!("{:?}", solution.unwrap());
    }
}
