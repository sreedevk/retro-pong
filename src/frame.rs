use crate::graphics::PIXOFF;
pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 32;

pub type Buffer = Vec<Vec<char>>;
pub struct Frame {
    pub memory: Buffer,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            memory: vec![vec![PIXOFF; WIDTH]; HEIGHT],
        }
    }
}
