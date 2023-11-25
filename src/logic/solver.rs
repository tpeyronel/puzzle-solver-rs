use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    sync::atomic::{self, AtomicBool},
    thread::{self, JoinHandle},
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
    ConnectionCheck,
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

struct Solver<'a> {
    threadi: u32,
    board: Board,
    total_remaining: u32,
    solution: SolutionWithBorrows<'a>,
    solver_tx: mpsc::UnboundedSender<SolverMessage>,
    should_report: &'a AtomicBool,
}

impl<'a> Solver<'a> {
    fn new(
        threadi: u32,
        width: u32,
        height: u32,
        solver_tx: mpsc::UnboundedSender<SolverMessage>,
        should_report: &'a AtomicBool,
    ) -> Self {
        Self {
            threadi,
            board: Board::new(width, height),
            total_remaining: 0,
            solution: SolutionWithBorrows { placed_shapes: vec![] },
            solver_tx,
            should_report,
        }
    }

    fn solve(&mut self, candidates: &'a Vec<Candidate>, remaining: &mut Vec<u32>) {
        self.total_remaining = remaining.iter().sum();

        let start = Instant::now();
        let solved = self.solve_rec(candidates, remaining, Vec2::ZERO);
        let elapsed = start.elapsed();

        let _ = self.solver_tx.send(SolverMessage::SolverEnd { elapsed });

        if solved {
            let _ = self.solver_tx.send(SolverMessage::SolutionMessage(SolutionMessage {
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

            if self.solver_tx.send(message).is_err() {
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

pub struct SolveJob {
    solver_rx: Option<mpsc::UnboundedReceiver<SolverMessage>>,
    controller_thread: Option<JoinHandle<()>>,
}

impl Deref for SolveJob {
    type Target = mpsc::UnboundedReceiver<SolverMessage>;

    fn deref(&self) -> &Self::Target {
        self.solver_rx.as_ref().unwrap()
    }
}

impl DerefMut for SolveJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.solver_rx.as_mut().unwrap()
    }
}

impl Drop for SolveJob {
    fn drop(&mut self) {
        // Drop solver_rx, so that workers finish early
        self.solver_rx = None;

        // Wait for all threads to finish (by waiting for controller thread)
        let _ = self.controller_thread.take().unwrap().join();
    }
}

pub fn solve_async(width: u32, height: u32, shapes: Vec<Shape>) -> SolveJob {
    let (solver_tx, solver_rx) = mpsc::unbounded_channel::<SolverMessage>();

    let controller_thread = thread::spawn(move || {
        let (candidates, remaining) = shapes_to_candidates(shapes);

        let thread_count = rayon::current_num_threads(); // Also initializes global thread pool

        let solvers_done = AtomicBool::new(false);
        let solvers_done = &solvers_done;

        let should_report_vars: Vec<AtomicBool> = (0..thread_count).map(|_| AtomicBool::new(false)).collect();

        let candidates_ref = &candidates;
        let should_report_vars_ref = &should_report_vars;

        thread::scope(|s| {
            s.spawn(move || {
                while !solvers_done.load(atomic::Ordering::Relaxed) {
                    for sr in should_report_vars_ref {
                        sr.store(true, atomic::Ordering::Relaxed);
                    }

                    thread::sleep(UPDATE_INTERVAL);
                }
            });

            rayon::scope(move |s| {
                for x in 0..width {
                    for y in 0..height {
                        for (i, c) in candidates_ref.iter().enumerate() {
                            for v in &c.variations {
                                let solver_tx = solver_tx.clone();
                                let seed = vec![(Vec2 { x, y }, v.clone())];
                                let mut remaining = remaining.clone();
                                remaining[i] -= 1;
                                let should_report_vars_ref = should_report_vars_ref;

                                s.spawn(move |_| {
                                    let threadi = rayon::current_thread_index().unwrap();

                                    if solver_tx.send(SolverMessage::ConnectionCheck).is_err() {
                                        return;
                                    }

                                    let mut solver = Solver::new(
                                        threadi as u32,
                                        width,
                                        height,
                                        solver_tx,
                                        &should_report_vars_ref[threadi],
                                    );

                                    if !solver.try_seed(&seed) {
                                        return;
                                    }

                                    solver.solve(candidates_ref, &mut remaining);
                                });
                            }
                        }
                    }
                }
            });

            solvers_done.store(true, atomic::Ordering::Relaxed);
        });
    });

    SolveJob {
        controller_thread: Some(controller_thread),
        solver_rx: Some(solver_rx),
    }
}

pub fn solve(width: u32, height: u32, shapes: Vec<Shape>) -> Option<Solution> {
    let mut solve_job = solve_async(width, height, shapes);

    let mut elapsed_sum = Duration::ZERO;
    let mut count = 0;

    while let Some(msg) = solve_job.blocking_recv() {
        match msg {
            SolverMessage::SolutionMessage(SolutionMessage {
                payload: SolutionPayload::TotalSolution(solution),
                ..
            }) => {
                return Some(solution);
            }
            SolverMessage::SolverEnd { elapsed } => {
                elapsed_sum += elapsed;
                count += 1;
            }
            _ => continue,
        }
    }

    println!("average: {} ms", (elapsed_sum / count).as_millis());

    None
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

    // #[test]
    // fn solve_works_for_all_digits() {
    //     let digits = vec![
    //         digit0(),
    //         digit1(),
    //         digit2(),
    //         digit3(),
    //         digit4(),
    //         digit5(),
    //         digit6(),
    //         digit7(),
    //         digit8(),
    //         digit9(),
    //     ];

    //     let solution = solver::solve(6, 5, digits);

    //     println!("{:?}", solution.unwrap());
    // }
}
