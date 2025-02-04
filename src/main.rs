use std::vec;

pub struct Planet{
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
    mass: f32,
    radius: f32,
}

pub struct System{
    planets: Vec<Planet>,
}

impl  System {
    pub fn apply_physics(&mut self){
        for planet in &mut self.planets{
            for phys_planet in &mut self.planets{
                
            }
        }
    }
}



fn main() {
    println!("Hello, world!");
}
