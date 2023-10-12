use std::rc::Rc;

use super::shape_mesh::ShapeMesh;

#[derive(Debug, Clone)]
pub struct ShapeMetadata {
    pub id: Rc<String>,
    pub rot: u32,
    pub flipped: bool,
}

#[derive(Debug, Clone)]
pub struct Shape {
    mesh: ShapeMesh,
    metadata: ShapeMetadata,
}

impl Shape {
    pub fn new(id: String, mesh: ShapeMesh) -> Self {
        Self {
            mesh,
            metadata: ShapeMetadata {
                id: Rc::new(id),
                rot: 0,
                flipped: false,
            },
        }
    }

    pub fn rotated_ccw(&self) -> Self {
        Self {
            mesh: self.mesh.rotated_ccw(),
            metadata: ShapeMetadata {
                id: self.metadata.id.clone(),
                rot: (self.metadata.rot + 1) % 4,
                flipped: self.metadata.flipped,
            },
        }
    }

    pub fn flipped_hor(&self) -> Self {
        Self {
            mesh: self.mesh.flipped_hor(),
            metadata: ShapeMetadata {
                id: self.metadata.id.clone(),
                rot: self.metadata.rot,
                flipped: !self.metadata.flipped,
            },
        }
    }

    pub fn mesh(&self) -> &ShapeMesh {
        &self.mesh
    }

    pub fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}
