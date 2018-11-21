use graphics::math::*;
use graphics::*;

mod bridge;
mod direction;

pub use self::bridge::*;
pub use self::direction::*;

macro_rules! t {
    ($x:ident) => {
        $x as f64 * 64.
    };
    (*$x:ident) => {
        *$x as f64 * 64.
    };
}

pub fn draw_body<G: Graphics>(
    body: &Vec<(usize, usize)>,
    c: &Context,
    transform: Matrix2d,
    g: &mut G,
    winfo: &crate::app::window_info::WindowInfoCache,
    color: [f32; 4]
) {
    let rect = rectangle::Rectangle::new(color);
    let color_low = [color[0] * 0.5, color[1] * 0.5, color[2] * 0.5, 1.];
    if winfo.no_moves != 0 {
        let mut iterator = body.iter().enumerate().skip(1).peekable();
        while let Some((i, (x, y))) = iterator.next() {
            if iterator.peek().is_none() {
                break;
            }
            let width = 15. * (1. - (i as f64 / body.len() as f64)) + 15.;
            let dir1 = Direction::find_dir(body[i - 1], (*x, *y), Some(winfo));
            let dir2 = Direction::find_dir((*x, *y), body[i + 1], Some(winfo));
            if let Some((ox, oy, sa, ea)) = Direction::get_angle_for_turn(dir2, dir1) {
                circle_arc(
                    color,
                    width,
                    sa,
                    ea,
                    [0., 0., 60., 60.],
                    transform.trans(t!(*x) + ox, t!(*y) + oy),
                    g,
                )
            } else {
                let width = 30. * (1. - (i as f64 / body.len() as f64)) + 30.;
                let dir_to_test = if dir1 == Direction::Middle {
                    dir2
                } else {
                    dir1
                };
                let rect_bounds = match dir_to_test {
                    Direction::Up | Direction::Down => {
                        [t!(*x) + (60. - width) * 0.5, t!(*y), width, 60.]
                    }
                    Direction::Left | Direction::Right => {
                        [t!(*x), t!(*y) + (60. - width) * 0.5, 60., width]
                    }
                    Direction::Middle => [0., 0., 0., 0.],
                };
                rect.draw(rect_bounds, &c.draw_state, transform, g);
            }
            if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
                draw_ghost((*x, *y), transform, g, winfo, color_low);
            }
        }
    }
    let (x, y) = body.last().unwrap();
    ellipse(
        color,
        [t!(*x) + 5., t!(*y) + 5., 50., 50.],
        transform,
        g,
    );
    if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
        draw_ghost((*x, *y), transform, g, winfo, color_low);
    }
}

fn draw_ghost<G: Graphics>(
    (x, y): (usize, usize),
    transform: Matrix2d,
    g: &mut G,
    winfo: &crate::app::window_info::WindowInfoCache,
    color: [f32; 4]
) {
    let mut flagx = true;
    let x = if x == 0 {
        winfo.grid_size.0 as f64
    } else if x == winfo.grid_size.0 - 1 {
        -0.5
    } else {
        flagx = false;
        x as f64
    } * 64.
        - 4.;
    let mut flagy = true;
    let y = if y == 0 {
        winfo.grid_size.1 as f64
    } else if y == winfo.grid_size.1 - 1 {
        -0.5
    } else {
        flagy = false;
        y as f64
    } * 64.
        - 4.;
    if flagx && flagy {
        return;
    } else if flagy {
        rectangle(color, [x, y, 64., 32.], transform, g);
    }
    if flagx {
        rectangle(color, [x, y, 32., 64.], transform, g);
    }
}

pub fn draw_apple<G: Graphics>((x, y): (usize, usize), transform: Matrix2d, g: &mut G) {
    ellipse(
        [1., 0., 0., 1.],
        rectangle::square(0., 0., 60.),
        transform.trans(t!(x), t!(y)),
        g,
    );
}

pub fn draw_bridges<G: Graphics>(bridges: &Vec<Bridge>, transform: Matrix2d, g: &mut G) {
    for Bridge { pos: (x, y) } in bridges.iter() {
        let x = t!(*x);
        let y = t!(*y);
        rectangle([0., 1., 0., 1.], [x + 2., y, 56., 60.], transform, g);
        rectangle([0., 0., 1., 1.], [x, y + 2., 60., 56.], transform, g);
        rectangle([0., 0., 0., 1.], [x + 2., y + 2., 56., 56.], transform, g);
    }
}
