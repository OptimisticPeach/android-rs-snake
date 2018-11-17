extern crate rand;

use super::snake;
use std::f64;

pub fn rand_range(start: usize, end: usize) -> u32 {
    let mut num: usize = rand::random();
    num %= end - start;
    num += start;
    num as u32
}
pub fn get_dir(rotation: f64, prev_dir: snake::Direction) -> snake::Direction {
    match (rotation * (2. / f64::consts::PI)).round() as isize % 4 {
        0 => {
            if prev_dir != snake::Direction::Left {
                snake::Direction::Right
            } else {
                prev_dir
            }
        }
        1 => {
            if prev_dir != snake::Direction::Down {
                snake::Direction::Up
            } else {
                prev_dir
            }
        }
        2 => {
            if prev_dir != snake::Direction::Right {
                snake::Direction::Left
            } else {
                prev_dir
            }
        }
        _ => {
            if prev_dir != snake::Direction::Up {
                snake::Direction::Down
            } else {
                prev_dir
            }
        }
    }
}
use super::snake::Direction::*;
use std::f64::consts::{FRAC_PI_2 as quarter, PI as half};
const THREE_QUARTERS: f64 = half + quarter;
const WHOLE: f64 = half + half;
pub fn get_angle_for_turn(
    old: snake::Direction,
    new: snake::Direction,
) -> Option<(f64, f64, f64, f64)> {
    if old == new || old == Middle || new == Middle {
        return None;
    }
    match old {
        Up => match new {
            Right => {
                return Some((30., 30., half, THREE_QUARTERS));
            }
            Left => {
                return Some((-30., 30., THREE_QUARTERS, WHOLE));
            }
            _ => {}
        },
        Down => match new {
            Right => {
                return Some((30., -30., quarter, half));
            }
            Left => {
                return Some((-30., -30., 0., quarter));
            }
            _ => {}
        },
        Left => match new {
            Up => {
                return Some((30., -30., quarter, half));
            }
            Down => {
                return Some((30., 30., half, THREE_QUARTERS));
            }
            _ => {}
        },
        Right => match new {
            Up => {
                return Some((-30., -30., 0., quarter));
            }
            Down => {
                return Some((-30., 30., THREE_QUARTERS, WHOLE));
            }
            _ => {}
        },
        _ => {}
    }
    None
}

use self::snake::Direction;

pub fn inverse(dir: Direction) -> Direction {
    match dir {
        Up => Down,
        Down => Up,
        Left => Right,
        Right => Left,
        Middle => Middle,
    }
}

pub fn get_corner_square(before: Direction, now: Direction) -> graphics::types::Rectangle {
    if before == now || now == inverse(before) || before == inverse(now) {
        match now {
            //x, y, w, h
            Up => [0., 30., 60., 30.],
            Down => [0., 0., 60., 30.],
            Left => [30., 0., 30., 60.],
            Right => [0., 0., 30., 60.],
            Middle => [0., 0., 0., 0.],
        }
    } else if now == Middle || before == Middle {
        [0., 0., 0., 0.]
    } else {
        match before {
            Up => match now {
                Left => [0., 30., 30., 30.],
                Right => [30., 30., 30., 30.],
                _ => unimplemented!("error dirnow is {:?} and before is {:?}", now, before),
            },
            Down => match now {
                Left => [0., 0., 30., 30.],
                Right => [30., 0., 30., 30.],
                _ => unimplemented!("error dirnow is {:?} and before is {:?}", now, before),
            },
            Left => match now {
                Up => [30., 0., 30., 30.],
                Down => [30., 30., 30., 30.],
                _ => unimplemented!("error dirnow is {:?} and before is {:?}", now, before),
            },
            Right => match now {
                Up => [0., 0., 30., 30.],
                Down => [0., 30., 30., 30.],
                _ => unimplemented!("error dirnow is {:?} and before is {:?}", now, before),
            },
            _ => unimplemented!(
                "error dirnow is {:?} and before is {:?} and failed at before",
                now,
                before
            ),
        }
    }
}

pub fn find_dir(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    winfo: Option<&super::window_info::WindowInfoCache>,
) -> snake::Direction {
    let (x1, y1, x2, y2) = (x1 as isize, y1 as isize, x2 as isize, y2 as isize);
    let dx = x1 - x2;
    let dy = y1 - y2;
    if let Some(winfo) = winfo {
        if dx == (winfo.grid_size.0 - 1) as isize {
            return Left;
        } else if dx == -(winfo.grid_size.0 as isize - 1) {
            return Right;
        } else if dy == (winfo.grid_size.1 - 1) as isize {
            return Up;
        } else if dy == -(winfo.grid_size.1 as isize - 1) {
            return Down;
        }
    }
    if dx > 0 {
        Right
    } else if dx < 0 {
        Left
    } else if dy > 0 {
        Down
    } else if dy < 0 {
        Up
    } else {
        Middle
    }
}
