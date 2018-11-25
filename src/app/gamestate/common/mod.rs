use graphics::math::*;
use graphics::*;

mod bridge;
mod counter;
mod direction;
mod touch;
mod triangle_cache;

pub use self::bridge::*;
pub use self::counter::*;
pub use self::direction::*;
pub use self::touch::*;
pub use self::triangle_cache::*;

macro_rules! t {
    ($x:ident) => {
        $x as f64 * 64.
    };
    (*$x:ident) => {
        *$x as f64 * 64.
    };
}

pub fn calc_body(
    body: &Vec<(usize, usize)>,
    transform: Matrix2d,
    winfo: &crate::app::window_info::WindowInfoCache,
    color: [f32; 4],
    tri_cache: &mut SnakeTriangleCache,
) {
    //Some setup
    tri_cache.body_colour = color;
    tri_cache.body.clear();
    tri_cache.ghosts.clear();
    tri_cache.ghosts_colour = [color[0] * 0.5, color[1] * 0.5, color[2] * 0.5, 1.];

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
                graphics::triangulation::with_arc_tri_list(
                    sa,
                    ea,
                    80,
                    transform.trans(t!(*x) + ox, t!(*y) + oy),
                    [0., 0., 60., 60.],
                    width,
                    |points| {
                        tri_cache.body.extend_from_slice(points);
                    },
                );
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
                tri_cache.body.extend_from_slice(
                    &graphics::triangulation::rect_tri_list_xy(transform, rect_bounds)[..],
                );
            }
            if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
                calc_ghost((*x, *y), transform, winfo, tri_cache);
            }
        }
    }
    let (x, y) = body.last().unwrap();

    graphics::triangulation::with_ellipse_tri_list(
        80,
        transform,
        [t!(*x) + 5., t!(*y) + 5., 50., 50.],
        |points| {
            tri_cache.body.extend_from_slice(points);
        },
    );

    if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
        calc_ghost((*x, *y), transform, winfo, tri_cache);
    }
}

fn calc_ghost(
    (x, y): (usize, usize),
    transform: Matrix2d,
    winfo: &crate::app::window_info::WindowInfoCache,
    tri_cache: &mut SnakeTriangleCache,
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
        tri_cache.ghosts.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform, [x, y, 64., 32.])[..],
        );
    }
    if flagx {
        tri_cache.ghosts.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform, [x, y, 32., 64.])[..],
        );
    }
}

pub fn calc_apple((x, y): (usize, usize), transform: Matrix2d, tri_cache: &mut TriangleCache) {
    tri_cache.apple.clear();
    tri_cache.apple_colour = [1., 0., 0., 1.];
    graphics::triangulation::with_ellipse_tri_list(
        80,
        transform.trans(t!(x), t!(y)),
        rectangle::square(0., 0., 60.),
        |points| {
            tri_cache.apple.extend_from_slice(points);
        },
    );
}

pub fn calc_bridges(bridges: &Vec<Bridge>, transform: Matrix2d, tri_cache: &mut TriangleCache) {
    tri_cache.bridges_green.clear();
    tri_cache.bridges_blue.clear();
    tri_cache.bridges_black.clear();
    for Bridge { pos: (x, y) } in bridges.iter() {
        let x = t!(*x);
        let y = t!(*y);
        tri_cache.bridges_green.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform, [x + 2., y, 56., 60.])[..],
        );
        tri_cache.bridges_blue.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform, [x, y + 2., 60., 56.])[..],
        );
        tri_cache.bridges_black.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform, [x + 2., y + 2., 56., 56.])[..],
        );
    }
}

pub fn calc_head((x, y): (usize, usize), other: (usize, usize), dir: Direction, transform: Matrix2d, tri_cache: &mut SnakeTriangleCache) {
    tri_cache.head.clear();
    let rect = Direction::get_corner_square(
        Direction::find_dir((x, y), other, None),
        dir,
    );
    tri_cache.head.extend_from_slice(
        &graphics::triangulation::rect_tri_list_xy(transform.trans(t!(x), t!(y)), rect)[..],
    );
    graphics::triangulation::with_ellipse_tri_list(
        80,
        transform.trans(t!(x), t!(y)),
        rectangle::square(0., 0., 60.),
        |points| {
            tri_cache.head.extend_from_slice(points);
        },
    );
}
