use super::*;

pub type TriList = Vec<[f32; 2]>;
pub type Color = [f32; 4];

pub struct SnakeTriangleCache {
    pub body: TriList,
    pub body_colour: Color,
    pub head: TriList,
    pub head_colour: Color,
    pub ghosts: TriList,
    pub ghosts_colour: Color,
}

macro_rules! t {
    ($x:ident) => {
        $x as f64 * 64.
    };
    (*$x:ident) => {
        *$x as f64 * 64.
    };
}

impl SnakeTriangleCache {
    pub fn new() -> Self {
        Self {
            body: Vec::new(),
            body_colour: [1.; 4],
            head: Vec::new(),
            head_colour: [1.; 4],
            ghosts: Vec::new(),
            ghosts_colour: [1.; 4],
        }
    }
    pub fn calc_head(
        &mut self,
        (x, y): (usize, usize),
        other: (usize, usize),
        dir: Direction,
        transform: Matrix2d,
    ) {
        self.head.clear();
        let rect = Direction::get_corner_square(Direction::find_dir((x, y), other, None), dir);
        self.head.extend_from_slice(
            &graphics::triangulation::rect_tri_list_xy(transform.trans(t!(x), t!(y)), rect)[..],
        );
        graphics::triangulation::with_ellipse_tri_list(
            80,
            transform.trans(t!(x), t!(y)),
            rectangle::square(0., 0., 60.),
            |points| {
                self.head.extend_from_slice(points);
            },
        );
    }
    pub fn calc_body(
        &mut self,
        body: &Vec<(usize, usize)>,
        transform: Matrix2d,
        winfo: &crate::app::window_info::WindowInfoCache,
        color: [f32; 4],
    ) {
        //Some setup
        self.body_colour = color;
        self.body.clear();
        self.ghosts.clear();
        self.ghosts_colour = [color[0] * 0.5, color[1] * 0.5, color[2] * 0.5, 1.];

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
                            self.body.extend_from_slice(points);
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
                    self.body.extend_from_slice(
                        &graphics::triangulation::rect_tri_list_xy(transform, rect_bounds)[..],
                    );
                }
                if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1
                {
                    self.calc_ghost((*x, *y), transform, winfo);
                }
            }
        }
        let (x, y) = body.last().unwrap();

        graphics::triangulation::with_ellipse_tri_list(
            80,
            transform,
            [t!(*x) + 5., t!(*y) + 5., 50., 50.],
            |points| {
                self.body.extend_from_slice(points);
            },
        );

        if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
            self.calc_ghost((*x, *y), transform, winfo);
        }
    }
    fn calc_ghost(
        &mut self,
        (x, y): (usize, usize),
        transform: Matrix2d,
        winfo: &crate::app::window_info::WindowInfoCache,
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
            self.ghosts.extend_from_slice(
                &graphics::triangulation::rect_tri_list_xy(transform, [x, y, 64., 32.])[..],
            );
        }
        if flagx {
            self.ghosts.extend_from_slice(
                &graphics::triangulation::rect_tri_list_xy(transform, [x, y, 32., 64.])[..],
            );
        }
    }
}

pub struct TriangleCache {
    pub apple: TriList,
    pub apple_colour: Color,
    pub snakes: Vec<SnakeTriangleCache>,
    pub bridges_green: TriList,
    pub bridges_blue: TriList,
    pub bridges_black: TriList,
}

impl TriangleCache {
    pub fn new() -> Self {
        Self {
            apple: Vec::new(),
            apple_colour: [1.; 4],
            snakes: Vec::new(),
            bridges_black: Vec::new(),
            bridges_blue: Vec::new(),
            bridges_green: Vec::new(),
        }
    }
    pub fn draw_all(&self, c: &Context, g: &mut impl Graphics) {
        g.tri_list(&c.draw_state, &self.apple_colour, |f| {
            f(self.apple.as_slice());
        });
        g.tri_list(&c.draw_state, &[0., 1., 0., 1.], |f| {
            f(self.bridges_green.as_slice());
        });
        g.tri_list(&c.draw_state, &[0., 0., 1., 1.], |f| {
            f(self.bridges_blue.as_slice());
        });
        g.tri_list(&c.draw_state, &[0., 0., 0., 1.], |f| {
            f(self.bridges_black.as_slice());
        });
        for i in self.snakes.iter() {
            g.tri_list(&c.draw_state, &i.body_colour, |f| {
                f(i.body.as_slice());
            });
            g.tri_list(&c.draw_state, &i.head_colour, |f| {
                f(i.head.as_slice());
            });
            g.tri_list(&c.draw_state, &i.ghosts_colour, |f| {
                f(i.ghosts.as_slice());
            });
        }
    }

    pub fn calc_apple(&mut self, (x, y): (usize, usize), transform: Matrix2d) {
        self.apple.clear();
        self.apple_colour = [1., 0., 0., 1.];
        graphics::triangulation::with_ellipse_tri_list(
            80,
            transform.trans(t!(x), t!(y)),
            rectangle::square(0., 0., 60.),
            |points| {
                self.apple.extend_from_slice(points);
            },
        );
    }

    pub fn calc_bridges(&mut self, bridges: &Vec<Bridge>, transform: Matrix2d) {
        self.bridges_green.clear();
        self.bridges_blue.clear();
        self.bridges_black.clear();
        for Bridge { pos: (x, y) } in bridges.iter() {
            let x = t!(*x);
            let y = t!(*y);
            self.bridges_green.extend_from_slice(
                &graphics::triangulation::rect_tri_list_xy(transform, [x + 2., y, 56., 60.])[..],
            );
            self.bridges_blue.extend_from_slice(
                &graphics::triangulation::rect_tri_list_xy(transform, [x, y + 2., 60., 56.])[..],
            );
            self.bridges_black.extend_from_slice(
                &graphics::triangulation::rect_tri_list_xy(transform, [x + 2., y + 2., 56., 56.])[..],
            );
        }
    }
}
