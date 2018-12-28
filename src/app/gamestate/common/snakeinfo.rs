use android_base::graphics::math::*;
use super::*;

pub struct SnakeInfo {
    pub dir: Direction,
    pub body: Vec<(usize, usize)>,
    pub color_head: [f32; 4],
    pub color_body: [f32; 4],
    pub moved: bool,
}

impl SnakeInfo {
    pub fn new(len: usize, x: usize, y: usize, colorb: [f32; 4], colorh: [f32; 4]) -> Self {
        if len <= 1 {
            panic!("Body length cannot be 0");
        }

        let mut temp_body = Vec::new();
        for _ in 0..len {
            temp_body.push((x, y));
        }

        Self {
            dir: Direction::Middle,
            body: temp_body,
            color_body: colorb,
            color_head: colorh,
            moved: false,
        }
    }

    pub fn advance(
        &mut self,
        winfo: &crate::app::window_info::WindowInfoCache,
        bridges: &Vec<Bridge>,
    ) -> (usize, usize){
        let (mut new_x, mut new_y) = self.dir.add_to(self.body[0]);
        if bridges
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

        for i in (1..self.body.len()).rev() {
            let to_be_get = self.body[i - 1].clone();
            self.body[i] = to_be_get;
        }

        self.body[0] = (new_x, new_y);
        (new_x, new_y)
    }

    pub fn contains(&self, other: &Self) -> bool {
        for b in self.body.iter().skip(1) {
            if &other.body[0] == b {
                return true;
            }
        }
        false
    }

    pub fn contains_pos(&self, other: &(usize, usize)) -> bool {
        for b in self.body.iter() {
            if other == b {
                return true;
            }
        }
        false
    }

    pub fn calc_for_draw(
        &self,
        transform: Matrix2d,
        winfo: &crate::app::window_info::WindowInfoCache,
        cache: &mut SnakeTriangleCache,
    ) {
        cache.calc_body(&self.body, transform, winfo, self.color_body);
        cache.calc_head(self.body[0], self.body[1], self.dir, transform);
        cache.head_colour = self.color_head;
    }
}
