use crate::graphics::{PIXOFF, PIXON};
pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 32;

pub type Buffer = Vec<Vec<char>>;
pub struct Frame {
    pub memory: Buffer,
}

impl Frame {
    pub fn new() -> Self {
        let mut memory = vec![vec![PIXOFF; WIDTH]; HEIGHT];
        memory[0] = vec![PIXON; WIDTH];
        memory[HEIGHT - 1] = vec![PIXON; WIDTH];
        memory[1..HEIGHT - 1].iter_mut().for_each(|line| {
            line[0] = PIXON;
            line[WIDTH - 1] = PIXON;
        });

        Self { memory }
    }
}
