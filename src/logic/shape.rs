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

    // TODO: replace with rotated_ccw(), etc.
    pub fn from_metadata(mesh: ShapeMesh, metadata: ShapeMetadata) -> Self {
        Self { mesh, metadata }
    }

    pub fn mesh(&self) -> &ShapeMesh {
        &self.mesh
    }

    pub fn metadata(&self) -> &ShapeMetadata {
        &self.metadata
    }
}
