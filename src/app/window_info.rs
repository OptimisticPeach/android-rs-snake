use std::collections::HashMap;
use android_base::graphics;

#[derive(Debug)]
pub struct WindowInfoCache {
    pub window_size: (usize, usize),
    pub grid_size: (usize, usize),
    pub offsets: (f64, f64),
    pub frame: u128,
    pub margin: usize,
    pub header: usize,
    pub no_moves: usize,
    pub gridoffsets: (f64, f64),
    pub frames_per_move: u8, 
    text_size_cache: HashMap<(String, u32), (f32, f32)>
}

impl WindowInfoCache {
    pub fn new() -> Self {
        Self {
            window_size: (0, 0),
            grid_size: (0, 0),
            offsets: (0., 0.),
            frame: 0,
            margin: 30,
            header: 220,
            no_moves: 0,
            gridoffsets: (0., 0.),
            frames_per_move: 18,
            text_size_cache: HashMap::new()
        }
    }

    pub fn recalc_gridoffsets(&mut self) {
        self.gridoffsets = (
            self.offsets.0 + self.margin as f64,
            self.offsets.1 + self.header as f64 + self.margin as f64,
        );
    }

    pub fn reset(&mut self) {
        let height = self.window_size.1 - (self.margin * 2) - self.header;
        let width = self.window_size.0 - (self.margin * 2);
        let num_x = width / 64;
        let num_y = height / 64;
        let offset_x = (width as f64 - (num_x as f64 * 64.)) * 0.5;
        let offset_y = (height as f64 - (num_y as f64 * 64.)) * 0.5;

        self.offsets = (offset_x, offset_y);
        self.grid_size = (num_x, num_y);
        self.recalc_gridoffsets();
    }

    /// Return size of the text in pixels
    pub fn size_of_str<T: graphics::character::CharacterCache>(
        &mut self,
        cache: &mut T,
        string: &str,
        scale: u32,
    ) -> (f64, f64) {
        if let Some(size) = self.text_size_cache.get(&(string.to_string(), scale)){
            return (size.0 as f64, size.1 as f64);
        }
        let mut size_x = 0.;
        let mut size_y = 0.;
        for i in string.chars() {
            let character = cache.character(scale, i).ok().unwrap();
            size_x += character.size[0] + character.offset[0];
            if size_y < character.size[1] + character.offset[1] {
                size_y = character.size[1] + character.offset[1];
            }
        }
        self.text_size_cache.insert((string.to_string(), scale), (size_x as f32, size_y as f32));
        (size_x as f64, size_y as f64)
    }
}
