#[derive(Debug)]
pub struct Touch {
    pub first: Option<(f32, f32)>,
    pub second: Option<(f32, f32)>,
    pub id: Option<usize>,
}

impl Touch { 
    pub fn new() -> Self {
        Touch {
            first: None,
            second: None,
            id: None,
        }
    }

    pub fn start(&mut self, x: f32, y: f32, id: usize) {
        if let Some(selfid) = self.id {
            if id == selfid {
                self.first = Some((x, y));
            }
        } else {
            self.id = Some(id);

            self.first = Some((x, y));
        }
    }

    fn get_angle((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f64 {
        ((-y1 + y2).atan2(x1 - x2) + std::f32::consts::PI) as f64
    }

    pub fn end(&mut self, x: f32, y: f32, reset: bool) -> Option<f64> {
        let angle;
        match self.first {
            Some(start) => {
                angle = Self::get_angle(start, (x, y));
            }
            None => return None,
        }

        if reset {
            self.first = None;
            self.id = None;
        } else {
            self.second = Some((x, y));
        }

        Some(angle)
    }

    pub fn cancel(&mut self) {
        if self.first.is_some() && self.second.is_none() {
            self.first = None;
        }
    }
}
