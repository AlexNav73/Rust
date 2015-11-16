
extern crate rand;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;

use aim;

struct State {
    /// Aim scaling speed
    a_scale_speed: f64,
    score: u32,
    // Is game over condition
    // Level 
    // Hp
}

impl State {

    pub fn set_scale_speed(&mut self, speed: f64) -> &mut Self {
        self.a_scale_speed = speed;
        self
    }

    pub fn add_points(&mut self, points: u32) -> &mut Self {
        self.score += points;
        self
    }

}

pub struct Settings {
    /// Window width
    w_wigth: u32,
    /// Window height
    w_height: u32,
    /// Aim minimal diameter
    a_min_d: f64,
    /// Aim maximal diameter
    a_max_d: f64,
    /// Game state
    state: State,
}

impl Settings {
    pub fn new(window: &[u32; 2]) -> Self {
        Settings {
            w_wigth: window[0],
            w_height: window[1],
            a_min_d: 20.0f64,
            a_max_d: 60.0f64,
            state: State {
                a_scale_speed: 0.06f64,
                score: 0,
            }
        }
    }

    pub fn set_min_diameter(&mut self, min: f64) -> &mut Self {
        self.a_min_d = min;
        self
    }

    pub fn set_max_diameter(&mut self, max: f64) -> &mut Self {
        self.a_max_d = max;
        self
    }
}

fn distance(aim: &mut aim::Aim, args: &[f64; 2]) -> f64 {
    let (mut x, mut y) = aim.get_location();
    x += aim.get_diameter() / 2.0;
    y += aim.get_diameter() / 2.0;
    let dx = (args[0] - x).abs();
    let dy = (args[1] - y).abs();
    (dx * dx + dy * dy).sqrt()
}

pub struct Game<'a> {
    settings: &'a mut Settings,
    aims: Vec<aim::Aim>,
}

impl<'a> Game<'a> {

    pub fn new(settings: &'a mut Settings) -> Game { 
        Game { 
            aims: vec![],
            settings: settings,
        }
    }

    pub fn add_aim(&mut self, color: [f32; 4], attr: [f64; 3]) {
        self.aims.push(aim::Aim::new(color, attr));
    }

    pub fn get_rand_loc(&self) -> [f64; 3] {
        use self::rand::Rng;

        let mut rnd = rand::thread_rng();
        let d = rnd.gen_range::<f64>(
            self.settings.a_min_d, self.settings.a_max_d);
        let right_x = self.settings.w_wigth as f64 - d;
        let bottom_y = self.settings.w_height as f64 - d;
        [rnd.gen_range::<f64>(0f64, right_x), 
        rnd.gen_range::<f64>(0f64, bottom_y), d]
    }

    pub fn update(&mut self, _dt: f64) {
        let attr = self.get_rand_loc();
        for aim in self.aims.iter_mut() {
            if aim.get_diameter() - 1.0 > 0.0f64 {
                let new_d = aim.get_diameter() 
                            - self.settings.state.a_scale_speed;
                aim.set_diameter(new_d);
            } else {
                aim.set_location(attr[0], attr[1]);
                aim.set_diameter(attr[2]);
            }
        }
    }

    pub fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        gl.draw(args.viewport(), |c, g| {
            clear([1.0; 4], g);

            for aim in self.aims.iter() {
                let (x, y) = aim.get_location();
                ellipse(*aim.color(), 
                        [x, y,
                        aim.get_diameter(), aim.get_diameter()],
                        c.transform, g);
            }
        });
    }
    

    pub fn mouse_press(&mut self, args: &[f64; 2]) {
        let mut pushed: usize = 0;
        let mut is_catched: bool = false;

        for aim in self.aims.iter_mut() {
            let dist = distance(aim, args);
            if dist < (aim.get_diameter() / 2.0) {
                is_catched = true;
                break;
            }
            pushed += 1;
        }

        if is_catched {
            let attr = self.get_rand_loc();
            self.aims[pushed].set_location(attr[0], attr[1]);
            self.aims[pushed].set_diameter(attr[2]);
            self.settings.state.add_points(
                (10.0 * self.aims[pushed].get_max_d() / 
                self.aims[pushed].get_diameter()) as u32);

            // TODO: Set correct formula to count game score
            
            println!("{}", (20.0 * self.aims[pushed].get_max_d() / 
                            self.aims[pushed].get_diameter()) as u32);
            println!("Score: {}", self.settings.state.score);
        }
    }

} // End of game impl

