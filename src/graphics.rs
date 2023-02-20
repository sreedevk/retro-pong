use crate::{frame::{Frame, HEIGHT, WIDTH}, score::Sprite};

pub const PIXON: char = '█';
pub const PIXOFF: char = ' ';

pub fn draw_line(frame: &mut Frame, y1: isize, x1: isize, y2: isize, x2: isize) {
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 as isize - y1 as isize).abs();
    let sx: isize = if x1 < x2 { 1 } else { -1 };
    let sy: isize = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x: isize = x1;
    let mut y: isize = y1;

    while x != x2 || y != y2 {
        if x < frame.memory.len() as isize && y < frame.memory[x as usize].len() as isize {
            frame.memory[x as usize][y as usize] = PIXON;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    if x < frame.memory.len() as isize && y < frame.memory[x as usize].len() as isize {
        frame.memory[x as usize][y as usize] = PIXOFF;
    }
}

pub fn draw_rect(frame: &mut Frame, x: usize, y: usize, width: usize, height: usize) {
    for i in x..(x + width) {
        for j in y..(y + height) {
            if i < HEIGHT && j < WIDTH {
                frame.memory[i][j] = PIXON;
            }
        }
    }
}

pub fn draw_sprite(frame: &mut Frame, sprite: Sprite, x: usize, y: usize) {
    for (iindex, i) in (x..(x + sprite.len())).enumerate() {
        for (jindex, j) in (y..(y + sprite.len())).enumerate() {
            if i < HEIGHT && j < WIDTH {
                frame.memory[i][j] = sprite[iindex][jindex];
            }
        }
    } 
}
