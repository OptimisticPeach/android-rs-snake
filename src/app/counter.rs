use graphics::*;

pub struct Counter {
    offset_x: f64,
    offset_y: f64,
    number: usize
}

impl Counter {
    pub fn new() -> Self {
        Self {
            offset_x: 0.,
            offset_y: 0.,
            number: 0,
        }
    }
    pub fn set_num<T: graphics::character::CharacterCache>(&mut self, num: usize, winfo: &mut super::window_info::WindowInfoCache, cache_ref: &mut T) {
        self.number = num;
        let size = winfo.size_of_str(cache_ref, &format!("{}", num), 80);
        self.offset_y = size.1 + 50.;
        self.offset_x = (winfo.window_size.0 as f64 - size.0) / 2.;
    }
    pub fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &self,
        c: &Context,
        cache_ref: &mut T,
        g: &mut G
    ) {
        text(
            [0.6, 0.3, 0.6, 1.],
            80,
            &format!("{}", self.number),
            cache_ref,
            c.transform.trans(self.offset_x, self.offset_y),
            g,
        );
    }
}
