use std::{f32::consts::PI};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector{
    pub angle: f32, 
    pub magnitude: f32,
}

// simplest particle form
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Joint {
    pub angle: f32,
    pub drag: f32,
    pub elasticity: f32,
    pub mass: f32,
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

impl ::core::ops::Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let dx = (self.angle.sin() * self.magnitude) + (other.angle.sin() * other.magnitude);
        let dy = (self.angle.cos() * self.magnitude) + (other.angle.cos() * other.magnitude);
        Vector {
            angle: 0.5 * PI - dy.atan2(dx),
            magnitude: dx.hypot(dy),
        }
    }
}

impl Joint {
    pub fn new(x: f32, y: f32, size: f32, mass: f32, speed: f32, angle: f32, elasticity: f32, drag: f32) -> Joint {
        Joint {
            x: x,
            y: y,
            size: size,
            mass: mass,
            speed: speed,
            angle: angle,
            elasticity: elasticity,
            drag: drag,
        }
    }
    
    pub fn accelerate(&mut self, acceleration: Vector) {
        let velocity = Vector { angle: self.angle, magnitude: self.speed, } + acceleration;
        self.angle = velocity.angle;
        self.speed = velocity.magnitude;
    }

    pub fn attract(&mut self, mut other: Joint) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = dx.hypot(dy);
        let theta = dy.atan2(dx);
        let force = 0.2 * self.mass * other.mass / distance.powi(2);
        self.accelerate(Vector {
            angle: theta - 0.5 * PI,
            magnitude: force / self.mass,
        });
        other.accelerate(Vector {
            angle: theta + 0.5 * PI,
            magnitude: force / other.mass,
        });
    }

    //bounce of enviroment walls
    pub fn bounce(&mut self, width: f32, height: f32) {
        if self.x > width - self.size{
            self.x = width - self.size;
            self.angle = -self.angle;
            self.speed *= self.elasticity;
        } else if self.x < 0.0 + self.size{
            self.x = 0.0 + self.size;
            self.angle = -self.angle;
            self.speed *= self.elasticity;
        }
        if self.y > height - self.size {
            self.y = height - self.size;
            self.angle = PI - self.angle;
            self.speed *= self.elasticity;
        } else if self.y < 0.0 + self.size {
            self.y = 0.0 + self.size;
            self.angle = PI - self.angle;
            self.speed *= self.elasticity;
        }
    }


    pub fn collide(&mut self, other: &mut Joint, dt: f32){
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = dx.hypot(dy);
        // honestly this should be it but ill stay true to the original fork for now
        if distance < self.size + other.size {
            let tangent = dy.atan2(dx);
            let angle = 0.5 * PI + tangent;
            let total_mass = self.mass + other.mass;

            let v1 = Vector {
                angle: self.angle,
                magnitude: self.speed * (self.mass - other.mass) / total_mass,
            } + Vector {
                angle: angle,
                magnitude: 2.0 * other.speed * other.mass / total_mass,
            };
            let v2 = Vector {
                angle: other.angle,
                magnitude: other.speed * (other.mass - self.mass) / total_mass,
            } + Vector {
                angle: angle + PI,
                magnitude: 2.0 * self.speed * self.mass / total_mass,
            };
            
            self.angle = v1.angle;
            self.speed = v1.magnitude;
            other.angle = v2.angle;
            other.speed = v2.magnitude;
                
            let new_elasticity = self.elasticity * other.elasticity;
                
            self.speed *= new_elasticity;
            other.speed *= new_elasticity;
                
            let overlap = 0.5 * (self.size + other.size - distance + 1.0);
                
            self.x += angle.sin() * overlap;
            self.y -= angle.cos() * overlap;
            other.x -= angle.sin() * overlap;
            other.y += angle.cos() * overlap;
            
        }
    }

    //combines the joint with another joint
    pub fn combine(&mut self, other: Joint) {
        let dx: f32 = self.x - other.x;
        let dy: f32 = self.y - other.y;
        let distance: f32 = dx.hypot(dy);

        if distance < (self.size + other.size) {
            let total_mass: f32 = self.mass + other.mass;
            self.x = (self.x * self.mass + other.x * other.mass) / total_mass;
            self.y = (self.y * self.mass + other.y * other.mass) / total_mass;
            let vector: Vector = Vector {
                angle: self.angle,
                magnitude: self.speed * self.mass / total_mass,
            } + Vector {
                angle: other.angle,
                magnitude: other.speed * other.mass / total_mass,
            };
            self.angle = vector.angle;
            self.speed = vector.magnitude;
            self.mass += total_mass;
        }
    }

    pub fn experience_drag(&mut self) {
        self.speed *= self.drag;
    }

    pub fn update(&mut self, dt: f32) {
        let speed = self.speed * 1.0;
        self.x += self.angle.sin() * speed;
        self.y -= self.angle.cos() * speed;
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        let dx = x - self.x;
        let dy = y - self.y;
        self.angle = dy.atan2(dx) + 0.5 * PI;
        self.speed = dx.hypot(dy) * 0.1;
    }
    
}
