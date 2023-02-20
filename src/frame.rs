use crate::graphics::PIXOFF;

pub type Buffer = Vec<Vec<char>>;
pub struct Frame {
    pub memory: Buffer,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            memory: vec![vec![PIXOFF; 128]; 32],
        }
    }
}
