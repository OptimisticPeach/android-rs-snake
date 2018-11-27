use graphics::math::*;
use graphics::*;

use super::super::common::*;

pub struct Snake {
    pub dir: Direction,
    pub apple: (usize, usize),
    pub body: Vec<(usize, usize)>,
    pub bridges: Vec<Bridge>,
    pub need_to_calc: bool,
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
            need_to_calc: true
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
        if winfo.frame % winfo.frames_per_move as u128 == 0 {
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
                return false; //didn't survive
            }
            for i in (1..self.body.len()).rev() {
                let to_be_get = self.body[i - 1].clone();
                self.body[i] = to_be_get;
            }
            self.body[0] = (new_x, new_y);
            if self.body[0] == self.apple {
                self.on_get_apple(winfo);
            }

            self.need_to_calc = true;
        }
        true //did survive
    }

    pub fn calc_draw(
        &mut self,
        c: &Context,
        g: &mut impl Graphics,
        transform: Matrix2d,
        winfo: &crate::app::window_info::WindowInfoCache,
        cache: &mut TriangleCache
    ) {
        if self.need_to_calc {
            if cache.snakes.len() == 0 {
                cache.snakes.push(SnakeTriangleCache::new());
                cache.snakes[0].head_colour = [0., 1., 1., 1.];
            }
            cache.snakes[0].calc_body(&self.body, transform, winfo, [0.4, 1., 0.4, 1.]);

            cache.snakes[0].calc_head(self.body[0], self.body[1], self.dir, transform);

            cache.calc_bridges(&self.bridges, transform);

            cache.calc_apple(self.apple, transform);

            self.need_to_calc = false;
        }
        cache.draw_all(c, g);
    }

    pub fn reset(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        *self = Self::new(4, 1, 1);
        self.reset_apple(winfo);
    }
}
