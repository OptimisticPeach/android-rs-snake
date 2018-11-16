use graphics::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
    Middle,
}

macro_rules! t {
    ($x:ident) => {
        ($x as f64 * 64.)
    };
    (*$x:ident) => {
        (*$x as f64 * 64.)
    };
}

impl Direction {
    pub fn add_to(&self, (x, y): (usize, usize)) -> (isize, isize) {
        let (x, y) = (x as isize, y as isize);
        let (x, y) = match *self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Middle => (x, y),
        };
        (x, y)
    }

    pub fn add_to_isize(&self, (x, y): (isize, isize)) -> (isize, isize) {
        let (x, y) = match *self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Middle => (x, y),
        };
        (x, y)
    }
}

pub struct Bridge {
    pub pos: (usize, usize),
}

impl Bridge {
    pub fn new(x: usize, y: usize) -> Self {
        Self { pos: (x, y) }
    }
}

impl PartialEq<(usize, usize)> for Bridge {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.pos.0 == other.0 && self.pos.1 == other.1
    }
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

    pub fn reset_apple(&mut self, winfo: &super::window_info::WindowInfoCache) {
        let mut posx = super::util::rand_range(0, winfo.grid_size.0) as usize;
        let mut posy = super::util::rand_range(0, winfo.grid_size.1) as usize;
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
            posx = super::util::rand_range(0, winfo.grid_size.0) as usize;
            posy = super::util::rand_range(0, winfo.grid_size.1) as usize;
        }
        self.apple = (posx, posy);
    }

    fn add_bridge(&mut self, winfo: &super::window_info::WindowInfoCache) {
        let mut posx = super::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
        let mut posy = super::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
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
            posx = super::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
            posy = super::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
        }
        self.bridges.push(Bridge::new(posx as usize, posy as usize));
    }

    fn on_get_apple(&mut self, winfo: &super::window_info::WindowInfoCache) {
        self.reset_apple(winfo);
        let last = self.body.last().unwrap().clone();
        self.body.push(last);
        self.body.push(last);
        self.body.push(last);
        if (self.body.len() - 4) % 15 == 0 {
            self.add_bridge(winfo);
        }
    }

    pub fn step(&mut self, winfo: &super::window_info::WindowInfoCache) -> bool {
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

    pub fn reset_pos(&mut self) {
        for i in self.body.iter_mut() {
            *i = (1, 1);
        }
    }

    fn draw_head<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        let (x, y) = self.body[0];
        let rect = match self.dir {
            //x, y, w, h
            Direction::Up => [
                winfo.gridoffsets.0 + t!(x),
                winfo.gridoffsets.1 + t!(y) + 30.,
                60.,
                30.,
            ],
            Direction::Down => [
                winfo.gridoffsets.0 + t!(x),
                winfo.gridoffsets.1 + t!(y),
                60.,
                30.,
            ],
            Direction::Left => [
                winfo.gridoffsets.0 + t!(x) + 30.,
                winfo.gridoffsets.1 + t!(y),
                30.,
                60.,
            ],
            Direction::Right => [
                winfo.gridoffsets.0 + t!(x),
                winfo.gridoffsets.1 + t!(y),
                30.,
                60.,
            ],
            Direction::Middle => [0., 0., 0., 0.],
        };
        rectangle::Rectangle::new([0., 1., 1., 1.]).draw(rect, &c.draw_state, c.transform, g);
        ellipse::Ellipse::new([0., 1., 1., 1.]).draw(
            rectangle::square(0., 0., 60.),
            &c.draw_state,
            c.transform
                .trans(winfo.gridoffsets.0 + t!(x), winfo.gridoffsets.1 + t!(y)),
            g,
        )
    }

    fn draw_apple<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        let (x, y) = self.apple;
        ellipse(
            [1., 0., 0., 1.],
            rectangle::square(0., 0., 60.),
            c.transform
                .trans(winfo.gridoffsets.0 + t!(x), winfo.gridoffsets.1 + t!(y)),
            g,
        );
    }

    fn draw_bridges<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        for Bridge { pos: (x, y) } in self.bridges.iter() {
            let x = winfo.gridoffsets.0 + t!(*x);
            let y = winfo.gridoffsets.1 + t!(*y);
            rectangle([0., 1., 0., 1.], [x + 2., y, 56., 60.], c.transform, g);
            rectangle([0., 0., 1., 1.], [x, y + 2., 60., 56.], c.transform, g);
            rectangle([0., 0., 0., 1.], [x + 2., y + 2., 56., 56.], c.transform, g);
        }
    }

    fn draw_ghost<G: Graphics>(
        (x, y): (usize, usize),
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
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
            + winfo.gridoffsets.0
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
            + winfo.gridoffsets.1
            - 4.;
        if flagx && flagy {
            return;
        } else if flagy {
            rectangle([0., 1., 0., 0.25], [x, y, 64., 32.], c.transform, g);
        }
        if flagx {
            rectangle([0., 1., 0., 0.25], [x, y, 32., 64.], c.transform, g);
        }
    }

    fn draw_body<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        let rect = rectangle::Rectangle::new([0.4, 1., 0.4, 1.]);
        if winfo.no_moves != 0 {
            let mut iterator = self.body.iter().enumerate().skip(1).peekable();
            while let Some((i, (x, y))) = iterator.next() {
                if iterator.peek().is_none() {
                    break;
                }
                let width = 15. * (1. - (i as f64 / self.body.len() as f64)) + 15.;
                let dir1 = super::util::find_dir(self.body[i - 1], (*x, *y), winfo);
                let dir2 = super::util::find_dir((*x, *y), self.body[i + 1], winfo);
                if let Some((ox, oy, sa, ea)) = super::util::get_corner_square(dir2, dir1) {
                    circle_arc(
                        [0.4, 1., 0.4, 1.],
                        width,
                        sa,
                        ea,
                        [0., 0., 60., 60.],
                        c.transform.trans(
                            winfo.gridoffsets.0 + t!(*x) + ox,
                            winfo.gridoffsets.1 + t!(*y) + oy,
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
                            winfo.gridoffsets.0 + t!(*x) + (60. - width) * 0.5,
                            winfo.gridoffsets.1 + t!(*y),
                            width,
                            60.,
                        ],
                        Direction::Left | Direction::Right => [
                            winfo.gridoffsets.0 + t!(*x),
                            winfo.gridoffsets.1 + t!(*y) + (60. - width) * 0.5,
                            60.,
                            width,
                        ],
                        Direction::Middle => [0., 0., 0., 0.],
                    };
                    rect.draw(rect_bounds, &c.draw_state, c.transform, g);
                }
                if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1
                {
                    Self::draw_ghost((*x, *y), c, g, winfo);
                }
            }
        }
        let (x, y) = self.body.last().unwrap();
        ellipse(
            [0.4, 1., 0.4, 1.],
            [
                winfo.gridoffsets.0 + t!(*x) + 5.,
                winfo.gridoffsets.1 + t!(*y) + 5.,
                50.,
                50.,
            ],
            c.transform,
            g,
        );
        if *x == 0 || *x == winfo.grid_size.0 - 1 || *y == 0 || *y == winfo.grid_size.1 - 1 {
            Self::draw_ghost((*x, *y), c, g, winfo);
        }
    }

    pub fn draw<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        self.draw_body(c, g, winfo);

        self.draw_head(c, g, winfo);

        self.draw_bridges(c, g, winfo);

        self.draw_apple(c, g, winfo);
    }

    pub fn reset(&mut self, winfo: &super::window_info::WindowInfoCache) {
        *self = Self::new(4, 1, 1);
        self.reset_apple(winfo);
    }
}
