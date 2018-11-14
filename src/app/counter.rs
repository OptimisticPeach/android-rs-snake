use graphics::*;

pub struct Counter {
    offset_x: f64,
    size_x: usize,
    bits: Vec<bool>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            offset_x: 0.,
            size_x: 0,
            bits: Vec::new(),
        }
    }
    pub fn set_num(&mut self, mut num: usize, winfo: &super::window_info::WindowInfoCache) {
        let mut bits = Vec::new();
        while num != 0 {
            bits.push(num % 2);
            num /= 2;
        }
        if bits.len() == 0 {
            bits.push(0);
        }
        self.bits = bits.iter().rev().map(|x| *x == 1).collect::<Vec<bool>>();
        self.size_x = (self.bits.len() * 129) - 1;
        let width = winfo.window_size.0;

        self.offset_x = (width - self.size_x) as f64 / 2.;
    }
    pub fn draw<G: Graphics>(
        &self,
        c: &Context,
        g: &mut G,
        winfo: &super::window_info::WindowInfoCache,
    ) {
        let rect_bounds = [
            self.offset_x + 32.,
            (winfo.header - 128) as f64 / 2. + 32.,
            64.,
            64.,
        ];
        for i in 0..self.bits.len() {
            let colour = if self.bits[i] {
                [0.6, 0.3, 0.6, 1.]
            } else {
                [0.2, 0.2, 0.2, 1.]
            };
            ellipse(
                colour,
                rect_bounds,
                c.transform.trans(i as f64 * 129., 0.),
                g,
            );
        }
    }
}
