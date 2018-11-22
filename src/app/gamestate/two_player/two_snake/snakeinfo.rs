use graphics::*;
use graphics::math::*;
use super::super::super::common::*;

macro_rules! t {
    ($x:ident) => {
        $x as f64 * 64.
    };
    (*$x:ident) => {
        *$x as f64 * 64.
    };
}

pub struct SnakeInfo{
    pub dir: Direction,
    pub body: Vec<(usize, usize)>,
    pub color_head: [f32; 4],
    pub color_body: [f32; 4],
    pub moved: bool
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
            moved: false
        }
    }

    pub fn advance(&mut self, winfo: &crate::app::window_info::WindowInfoCache, bridges: &Vec<Bridge>){
        let (mut new_x, mut new_y) = self.dir.add_to(self.body[0]);
        if   bridges
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
        
    }

    pub fn contains(&self, other: &Self) -> bool{
        for b in self.body.iter().skip(1){
            if &other.body[0] == b{
                return true
            }
        }
        false
    }

    pub fn contains_pos(&self, other: &(usize, usize)) -> bool{
        for b in self.body.iter(){
            if other == b{
                return true
            }
        }
        false
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
        rectangle::Rectangle::new(self.color_head).draw(
            rect,
            &c.draw_state,
            transform
                .trans(t!(x), t!(y)),
            g,
        );
        ellipse::Ellipse::new(self.color_head).draw(
            rectangle::square(0., 0., 60.),
            &c.draw_state,
            transform
                .trans(t!(x), t!(y)),
            g,
        )
    }

    pub fn draw<G: Graphics>(
        &self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        winfo: &crate::app::window_info::WindowInfoCache,
    ){
        draw_body(&self.body, c, transform, g, winfo, self.color_body);

        self.draw_head(c, transform, g);
    }
}