
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Planet{
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    mass: f32,
    radius: f32,
}

impl  Planet {
    pub fn new( x_pos:f64, y_pos:f64, x_vel:f64, y_vel:f64,mass:f32, radius:f32)-> Planet{
        Planet { x_pos: x_pos, y_pos: y_pos, x_vel: x_vel, y_vel: y_vel, mass: mass, radius: radius }
    }
}

pub struct System{
    planets: Vec<Planet>,
}

impl  System {
    pub fn new() -> System{
        System {planets: Vec::new()}
    }

    pub fn predict_orbit(&self, steps: usize, dt: f64) -> Vec<Vec<(f64, f64)>> {
        let mut planets: Vec<Planet> = self.planets.clone();
        let mut predictions: Vec<Vec<(f64, f64)>> = vec![Vec::new(); planets.len()];

        const GRAV: f64 = 0.66743015;

        for _ in 0..steps {
            let mut accelerations: Vec<(f64, f64)> = vec![(0.0, 0.0); planets.len()];

            for i in 0..planets.len() {
                for j in 0..planets.len() {
                    if i == j {
                        continue;
                    }

                    let dx = planets[j].x_pos - planets[i].x_pos;
                    let dy = planets[j].y_pos - planets[i].y_pos;
                    let distance_sq = dx * dx + dy * dy;

                    if distance_sq < 1.0 {
                        continue;
                    }


                    let force = GRAV * planets[j].mass as f64 / distance_sq;
                    let distance = distance_sq.sqrt();

                    accelerations[i].0 += force * (dx / distance);
                    accelerations[i].1 += force * (dy / distance);


                }
            }

            for (i, planet) in planets.iter_mut().enumerate() {
                planet.x_vel += accelerations[i].0 * dt;
                planet.y_vel += accelerations[i].1 * dt;
                planet.x_pos += planet.x_vel * dt;
                planet.y_pos += planet.y_vel * dt;
                predictions[i].push((planet.x_pos, planet.y_pos));
            }
        }

        predictions
    }

    pub fn apply_physics(&mut self, delta_time: f64){
        let len = self.planets.len();
    
        for i in 0..len {
            let (left, right) = self.planets.split_at_mut(i);
            let (planet1, rest) = right.split_first_mut().unwrap();
    
            let mut ax = 0.0;
            let mut ay = 0.0;
    
            for planet2 in left.iter().chain(rest.iter()) {
                let dx = planet2.x_pos - planet1.x_pos;
                let dy = planet2.y_pos - planet1.y_pos;
                let distance_sq = dx * dx + dy * dy;
    
                if distance_sq < 1.0 { continue; }  
    
                const GRAV: f64 = 0.66743015;
                let force = GRAV * planet2.mass as f64 / distance_sq;
    
                let distance = distance_sq.sqrt();
    
                ax += force as f64 * (dx / distance)as f64;
                ay += force as f64 * (dy / distance)as f64;
            }
    
            planet1.x_vel += ax * delta_time;
            planet1.y_vel += ay * delta_time;
    
            planet1.x_pos += planet1.x_vel * delta_time;
            planet1.y_pos += planet1.y_vel * delta_time;
        }
    }

    pub fn display(&mut self){
        for planet in &self.planets{
            draw_circle(planet.x_pos as f32, planet.y_pos as f32, planet.radius, WHITE);
            
        }
        
    }

    pub fn add_planet( &mut self, x_pos:f64, y_pos:f64, x_vel:f64, y_vel:f64,mass:f32, radius:f32){
        self.planets.push(Planet::new(x_pos, y_pos, x_vel, y_vel, mass, radius));   
    }
}


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut system = System::new();

    let mass_scale = 1e10;
    let distance_scale = 1e-9;
    let sim_time_multiplier: f64 = 1e-7;

        // Real-world values for the Sun
        let sun_mass = 1.989e30 / mass_scale; // [kg] scaled
        let sun_radius = 30.0;               // Arbitrary visual radius in pixels
        let sun_x = 400.0;
        let sun_y = 300.0;

        // Real-world values for Earth
        let earth_mass = 5.972e24 / mass_scale; // [kg] scaled
        let earth_radius = 5.0;                 // Visual radius in pixels
        let au = 1.496e11;                      // 1 AU in meters
        let earth_distance: f64 = au * distance_scale; // in pixels (~150 pixels)
        let earth_x = sun_x + earth_distance;
        let earth_y = sun_y;

        // Corrected velocity: perpendicular to radial direction
        let g_sim = 0.66743015;
        let earth_velocity = (g_sim * sun_mass / earth_distance).sqrt() * sim_time_multiplier;

        // Apply velocity perpendicular to radial vector
        let earth_vx = 0.0;
        let earth_vy = -earth_velocity * 1e7 ; // Moving "upward" relative to the screen

       system.add_planet(sun_x, sun_y, 0.0, 0.0, sun_mass as f32, sun_radius);
       system.add_planet(earth_x, earth_y, earth_vx, earth_vy, earth_mass as f32, earth_radius);
   

    loop {


        let dt = get_frame_time() as f64;
        let dt_effective = dt * sim_time_multiplier;
        clear_background(BLACK);
        system.display();
        system.apply_physics(dt_effective);
        let prediction_steps = 1000;

        for path in system.predict_orbit(prediction_steps, dt_effective) {
            for window in path.windows(2) {
                if let [p0, p1] = window {
                    draw_line(p0.0 as f32, p0.1 as f32, p1.0 as f32, p1.1 as f32, 1.0, RED);
                }
            }
        }

        next_frame().await
    }
}
