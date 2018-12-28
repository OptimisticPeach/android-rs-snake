use android_base::*;
use graphics::*;

pub struct Counter {
    offset_x: f64,
    offset_y: f64,
    number: usize,
    rotation: f64,
    others: usize,
}

impl Counter {
    pub fn new(rotation: f64, others: usize) -> Self {
        Self {
            offset_x: 0.,
            offset_y: 0.,
            number: 0,
            rotation: rotation,
            others: others
        }
    }
    pub fn set_num<T: graphics::character::CharacterCache>(&mut self, num: usize, winfo: &mut crate::app::window_info::WindowInfoCache, cache_ref: &mut T, which: usize) {
        self.number = num; 
        let size = winfo.size_of_str(cache_ref, &format!("{}", num), 100);
        self.offset_x = ((winfo.window_size.0 as f64 / (self.others as f64 + 1.)) * which as f64) - (size.0 / 2.);
        if self.others > 1 {
            if self.rotation > 0. {
                self.offset_y = size.1 + 50. - (size.0 * 0.5);
            } else {
                self.offset_y = size.1 + 50. + (size.0 * 0.5);
            }
        } else {
            self.offset_y = size.1 + 50.;
        }
    }
    pub fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &self,
        c: &Context,
        cache_ref: &mut T,
        g: &mut G
    ) {
        text(
            [0.6, 0.3, 0.6, 1.],
            100,
            &format!("{}", self.number),
            cache_ref,
            c.transform.trans(self.offset_x, self.offset_y).rot_rad(self.rotation),
            g,
        );
    }
}
