use graphics::*;
use graphics::math::*;

mod direction;
mod bridge;

pub use self::bridge::Bridge;
pub use self::direction::Direction;

macro_rules! t {
    ($x:ident) => {
        $x as f64 * 64.
    };
    (*$x:ident) => {
        *$x as f64 * 64.
    };
}

pub struct Snake {
    pub dir: Direction,
    pub apple: (usize, usize),
    pub body: Vec<(usize, usize)>,
    pub bridges: Vec<Bridge>,
}

impl Snake {
    pub fn new(len: usize, x: usize, y: usize) -> Self {
        if len <= 1 {
            panic!("Body length cannot be 0");
        }

        let mut temp_body = Vec::new();
        for _ in 0..len {
            temp_body.push((x, y));
        }

        Snake {
            dir: Direction::Middle,
            body: temp_body,
            apple: (0, 0),
            bridges: Vec::new(),
        }
    }

    pub fn reset_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let mut posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
        let mut posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        while {
            if self.body.contains(&(posx as usize, posy as usize)) {
                true
            } else {
                let mut flag = false;
                for Bridge { pos } in self.bridges.iter() {
                    if pos == &(posx, posy) {
                        flag = true;
                    }
                }
                flag
            }
        } {
            posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
            posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        }
        self.apple = (posx, posy);
    }

    fn add_bridge(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let mut posx = crate::app::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
        let mut posy = crate::app::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
        while {
            if self.apple == (posx as usize, posy as usize) {
                true
            } else if self.body.contains(&(posx as usize, posy as usize)) {
                true
            } else {
                let mut flag = false;
                for Bridge { pos: (x, y) } in self.bridges.iter() {
                    let (x, y) = (*x as isize, *y as isize);
                    if (x, y) == (posx, posy) {
                        flag = true;
                    } else if (x - 1, y) == (posx, posy)
                        || (x + 1, y) == (posx, posy)
                        || (x, y - 1) == (posx, posy)
                        || (x, y + 1) == (posx, posy)
                    {
                        flag = true;
                    }
                }
                flag
            }
        } {
            posx = crate::app::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
            posy = crate::app::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
        }
        self.bridges.push(Bridge::new(posx as usize, posy as usize));
    }

    fn on_get_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        self.reset_apple(winfo);
        let last = self.body.last().unwrap().clone();
        self.body.push(last);
        self.body.push(last);
        self.body.push(last);
        if (self.body.len() - 4) % 15 == 0 {
            self.add_bridge(winfo);
        }
    }

    pub fn step(&mut self, winfo: &crate::app::window_info::WindowInfoCache) -> bool {
        let (mut new_x, mut new_y) = self.dir.add_to(self.body[0]);
        if self
            .bridges
            .iter()
            .filter(|x| ((**x).pos.0 as isize, (**x).pos.1 as isize) == (new_x, new_y))
            .next()
            .is_some()
        {
            let (nx, ny) = self.dir.add_to_isize((new_x, new_y));
            new_x = nx;
            new_y = ny;
        }

        if new_x < 0 {
            new_x = winfo.grid_size.0 as isize - 1;
        } else if new_y < 0 {
            new_y = winfo.grid_size.1 as isize - 1;
        } else if new_x >= winfo.grid_size.0 as isize {
            new_x = 0;
        } else if new_y >= winfo.grid_size.1 as isize {
            new_y = 0;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        if self.body.contains(&(new_x as usize, new_y as usize)) && winfo.no_moves != 0 {
            return false;
        }
        for i in (1..self.body.len()).rev() {
            let to_be_get = self.body[i - 1].clone();
            self.body[i] = to_be_get;
        }
        self.body[0] = (new_x, new_y);
        if self.body[0] == self.apple {
            self.on_get_apple(winfo);
        }
        true
    }

    fn draw_head<G: Graphics>(
        &self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G
    ) {
        let (x, y) = self.body[0];
        let rect = Direction::get_corner_square(
            Direction::find_dir(self.body[0], self.body[1], None),
            self.dir,
        );
        rectangle::Rectangle::new([0., 1., 1., 1.]).draw(
            rect,
            &c.draw_state,
            transform
                .trans(t!(x), t!(y)),
            g,
        );
        ellipse::Ellipse::new([0., 1., 1., 1.]).draw(
            rectangle::square(0., 0., 60.),
            &c.draw_state,
            transform
                .trans(t!(x), t!(y)),
            g,
        )
    }

    fn draw_apple<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G
    ) {
        let (x, y) = self.apple;
        ellipse(
            [1., 0., 0., 1.],
            rectangle::square(0., 0., 60.),
            transform
                .trans(t!(x), t!(y)),
            g,
        );
    }

    fn draw_bridges<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G
    ) {
        for Bridge { pos: (x, y) } in self.bridges.iter() {
            let x = t!(*x);
            let y = t!(*y);
            rectangle([0., 1., 0., 1.], [x + 2., y, 56., 60.], transform, g);
            rectangle([0., 0., 1., 1.], [x, y + 2., 60., 56.], transform, g);
            rectangle([0., 0., 0., 1.], [x + 2., y + 2., 56., 56.], transform, g);
        }
    }

    fn draw_ghost<G: Graphics>(
        (x, y): (usize, usize),
        transform: Matrix2d,
        g: &mut G,
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
            rectangle([0., 1., 0., 0.25], [x, y, 64., 32.], transform, g);
        }
        if flagx {
            rectangle([0., 1., 0., 0.25], [x, y, 32., 64.], transform, g);
        }
    }

    fn draw_body<G: Graphics>(
        &self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        winfo: &crate::app::window_info::WindowInfoCache,
    ) {
        let rect = rectangle::Rectangle::new([0.4, 1., 0.4, 1.]);
        if winfo.no_moves != 0 {
            let mut iterator = self.body.iter().enumerate().skip(1).peekable();
            while let Some((i, (x, y))) = iterator.next() {
                if iterator.peek().is_none() {
                    break;
                }
                let width = 15. * (1. - (i as f64 / self.body.len() as f64)) + 15.;
                let dir1 = Direction::find_dir(self.body[i - 1], (*x, *y), Some(winfo));
                let dir2 = Direction::find_dir((*x, *y), self.body[i + 1], Some(winfo));
                if let Some((ox, oy, sa, ea)) = Direction::get_angle_for_turn(dir2, dir1) {
                    circle_arc(
                        [0.4, 1., 0.4, 1.],
                        width,
                        sa,
                        ea,
                        [0., 0., 60., 60.],
                        transform.trans(
                            t!(*x) + ox,
                            t!(*y) + oy,
                        ),
                        g,
                    )
                } else {
                    let width = 30. * (1. - (i as f64 / self.body.len() as f64)) + 30.;
                    let dir_to_test = if dir1 == Direction::Middle {
                        dir2
                    } else {
                        dir1
                    };
                    let rect_bounds = match dir_to_test {
                        Direction::Up | Direction::Down => [
                            t!(*x) + (60. - width) * 0.5,
                            t!(*y),
                            width,
                            60.,
                        ],
                        Direction::Left | Direction::Right => [
                            t!(*x),
                            t!(*y) + (60. - width) * 0.5,
                            60.,
                            width,
                        ],
                        Direction::Middle => [0., 0., 0., 0.],
                    };
                    rect.draw(rect_bounds, &c.draw_state, transform, g);
                }
                if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1
                {
                    Self::draw_ghost((*x, *y), transform, g, winfo);
                }
            }
        }
        let (x, y) = self.body.last().unwrap();
        ellipse(
            [0.4, 1., 0.4, 1.],
            [
                t!(*x) + 5.,
                t!(*y) + 5.,
                50.,
                50.,
            ],
            transform,
            g,
        );
        if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
            Self::draw_ghost((*x, *y), transform, g, winfo);
        }
    }

    pub fn draw<G: Graphics>(
        &self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        winfo: &crate::app::window_info::WindowInfoCache,
    ) {
        self.draw_body(c, transform, g, winfo);

        self.draw_head(c, transform, g);

        self.draw_bridges(transform, g);

        self.draw_apple(transform, g);
    }

    pub fn reset(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        *self = Self::new(4, 1, 1);
        self.reset_apple(winfo);
    }
}
