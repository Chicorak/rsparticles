use std::{f32::consts::PI};

use super::joint::{Joint, Vector};

#[derive(PartialEq, Debug)]
pub struct Spring{
    pub position_1: (f32, f32),
    pub position_2: (f32, f32),
    pub indices: (usize, usize),
    pub length: f32,
    pub strength: f32,
} 

impl Spring{
    pub fn new ( indices: (usize, usize), length: f32, strength: f32) -> Spring {
        Spring {
            position_1: (0.0, 0.0),
            position_2: (0.0, 0.0),
            indices,
            length,
            strength,
        }
    }

    pub fn update (&mut self, a: &Joint, b: &Joint) -> (Vector, Vector){
        self.position_1 = (a.x, a.y);
        self.position_2 = (b.x, b.y);

        let dx: f32 = a.x - b.x;
        let dy: f32 = a.y - b.y;
        
        let distance: f32 = dx.hypot(dy);
        let theta: f32 = dy.atan2(dx);
        let force: f32 = (self.length - distance) * self.strength;
        
        (Vector {
            angle: theta + 0.5 * PI,
            magnitude: force / a.mass,
        },
        
        Vector {
            angle: theta - 0.5 * PI,
            magnitude: force / a.mass,
        })
    }

}