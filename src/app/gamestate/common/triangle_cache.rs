use graphics::*;

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

impl SnakeTriangleCache {
    pub fn new() -> Self {
        Self{
            body: Vec::new(),
            body_colour: [1.; 4],
            head: Vec::new(),
            head_colour: [1.; 4],
            ghosts: Vec::new(),
            ghosts_colour: [1.; 4],
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
            f(&self.apple[..]);
        });
        g.tri_list(&c.draw_state, &[0., 1., 0., 1.], |f| {
            f(&self.bridges_green[..]);
        });
        g.tri_list(&c.draw_state, &[0., 0., 1., 1.], |f| {
            f(&self.bridges_blue[..]);
        });
        g.tri_list(&c.draw_state, &[0., 0., 0., 1.], |f| {
            f(&self.bridges_black[..]);
        });
        for i in self.snakes.iter() {
            g.tri_list(&c.draw_state, &i.body_colour, |f| {
                f(&i.body[..]);
            });
            g.tri_list(&c.draw_state, &i.head_colour, |f| {
                f(&i.head[..]);
            });
            g.tri_list(&c.draw_state, &i.ghosts_colour, |f| {
                f(&i.ghosts[..]);
            });
        }
    }
}
