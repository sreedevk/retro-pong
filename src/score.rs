use crate::graphics::{PIXOFF, PIXON};

pub type Sprite = [[char; 5]; 5];
pub const ZERO: Sprite = [
    [PIXON, PIXON, PIXON, PIXON, PIXON],
    [PIXON, PIXOFF, PIXOFF, PIXOFF, PIXON],
    [PIXON, PIXOFF, PIXOFF, PIXOFF, PIXON],
    [PIXON, PIXOFF, PIXOFF, PIXOFF, PIXON],
    [PIXON, PIXON, PIXON, PIXON, PIXON]
];
