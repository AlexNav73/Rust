
pub struct Aim {
    color: [f32; 4],
    x: f64,
    y: f64,
    diameter: f64,
    max_diameter: f64,
}

impl Aim {

    pub fn new(col: [f32; 4], loc: [f64; 3]) -> Self {
        Aim {
            color: col,
            x: loc[0],
            y: loc[1],
            diameter: loc[2],
            max_diameter: loc[2],
        }
    }

    pub fn get_diameter(&self) -> f64 {
        self.diameter
    }

    pub fn get_max_d(&self) -> f64 {
        self.max_diameter
    }

    pub fn set_diameter(&mut self, new_d: f64) {
        self.diameter = new_d;
    }

    pub fn color(&self) -> &[f32; 4] {
        &self.color
    } 

    pub fn get_location(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn set_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
