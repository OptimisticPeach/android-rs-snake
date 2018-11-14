pub struct WindowInfoCache {
    pub window_size: (usize, usize),
    pub grid_size: (usize, usize),
    pub offsets: (f64, f64),
    pub frame: u128,
    pub margin: usize,
    pub header: usize,
    pub no_moves: usize,
    pub gridoffsets: (f64, f64),
    pub camera_follow: bool,
    pub frames_per_move: u8
}

impl WindowInfoCache {
    pub fn new() -> Self {
        Self {
            window_size: (0, 0),
            grid_size: (0, 0),
            offsets: (0., 0.),
            frame: 0,
            margin: 20,
            header: 220,
            no_moves: 0,
            gridoffsets: (0., 0.),
            camera_follow: true,
            frames_per_move: 18
        }
    }

    pub fn recalc_gridoffsets(&mut self){
        self.gridoffsets = (self.offsets.0 + self.margin as f64, self.offsets.1 + self.header as f64 + self.margin as f64);
    }

    pub fn set_offsets(&mut self, offsets: (f64, f64)){
        self.offsets = offsets;
        self.recalc_gridoffsets();
    }
}
