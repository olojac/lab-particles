use crate::types::{Line, Position};

pub struct Bounds {
    pub width: f64,
    pub height: f64,
    pub top: Line,
    pub bottom: Line,
    pub left: Line,
    pub right: Line,
}

impl Bounds {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            top: (Position::new(0.0, 0.0), Position::new(width, 0.0)),
            bottom: (Position::new(width, height), Position::new(0.0, height)),
            left: (Position::new(0.0, height), Position::new(0.0, 0.0)),
            right: (Position::new(width, 0.0), Position::new(width, height)),
        }
    }

    pub fn candidates(&self, next_position: &Position) -> Vec<&Line> {
        let mut selected = vec![];

        if next_position.x < self.left.0.x {
            selected.push(&self.left);
        } else if next_position.x > self.right.0.x {
            selected.push(&self.right);
        }

        if next_position.y < self.top.0.y {
            selected.push(&self.top);
        } else if next_position.y > self.bottom.0.y {
            selected.push(&self.bottom);
        }

        selected
    }
}
