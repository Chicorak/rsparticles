use std::{f32::consts::PI, f32};

mod joint;
mod spring;
mod line;
use line::Line;
use joint::{Joint, Vector};
use spring::Spring;

pub struct Enviroment {
    //constants
    width: i32,
    height: i32,
    stable: f32,

    allow_acceleration: bool,
    allow_attract: bool,
    allow_bounce: bool,
    allow_collide: bool,
    allow_combine: bool,
    allow_drag: bool,
    allow_move: bool,
    airmass: f32,

    //vectors 
    joints: Vec<Joint>,
    springs: Vec<Spring>,
    lines: Vec<Line>,
    acceleration: Vector,
}

impl Enviroment {
    //default constructor (simplified)
    pub fn new( width: i32, height: i32) -> Enviroment{
        Enviroment {
            width: width,
            height: height,
            
            stable: 0.05,
            allow_acceleration: true,
            allow_attract: false,
            allow_bounce: true,
            allow_collide: true,
            allow_combine: false,
            allow_drag: false,
            allow_move: true,
            airmass: 0.2,
            
            acceleration: Vector{
                angle: PI, 
                magnitude: 0.9
            },
            joints: Vec::new(),
            springs: Vec::new(),
            lines: Vec::new(),
        }
    }   

    pub fn new_custom( height: i32, width: i32, stable: f32, allow_acceleration: bool, allow_attract: bool, allow_bounce: bool, allow_collide: bool, allow_combine: bool, allow_drag: bool, allow_move: bool, airmass: f32, acceleration: Vector) -> Enviroment {
        Enviroment {
            height,
            width,
            stable,
            allow_acceleration,
            allow_attract,
            allow_bounce,
            allow_collide,
            allow_combine,
            allow_drag,
            allow_move,
            airmass,
            acceleration,
            joints: Vec::new(),
            springs: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn get_joint(&mut self, index: usize) -> &mut Joint {
        &mut self.joints[index]
    }

    pub fn get_joint_at(&mut self, x: f32, y: f32) -> usize {
        for i in 0..self.joints.len() {
            //distance
            if (x - self.joints[i].x).hypot(y - self.joints[i].y) < self.joints[i].size {
                return i.try_into().unwrap();
            } 
        }
        self.joints.len()
    }

    pub fn get_spring(&mut self, index: usize) -> &mut Spring {
        &mut self.springs[index]
    }

    pub fn get_springs(&mut self) -> &mut Vec<Spring> {
        &mut self.springs
    }

    pub fn get_joints(&mut self) -> &mut Vec<Joint> {
        &mut self.joints
    }

    pub fn get_lines(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }


    pub fn add_joint(&mut self, x: f32, y: f32, size: f32, angle: f32, speed: f32) -> Joint {
        let mass = size * size;
        let drag = ((mass / (mass + self.airmass))).powf(2.0);
        let j = Joint::new(x, y, size, mass, speed, angle, 0.8, drag);
        self.joints.push(j);
        j
    }
    
    pub fn add_joint_custom(&mut self, x: f32, y: f32, size: f32, mass: f32, speed: f32, angle: f32, elasticity: f32) -> Joint {
        let drag = ((mass / (mass + self.airmass))).powf(size);
        let j = Joint::new(x, y, size, mass, speed, angle, elasticity, drag);
        self.joints.push(j);
        j
    }

    pub fn add_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> Line {
        let l = Line::new(x1, y1, x2, y2, width);
        self.lines.push(l);
        l
    }

    pub fn add_spring(&mut self, index_a: usize, index_b: usize, rest_length: f32, strength: f32) {
        let s = Spring::new((index_a, index_b), rest_length, strength);
        self.springs.push(s);
    }

    //remove joint
    
    pub fn remove_joint(&mut self, index: usize) {
        self.joints.remove(index);
    }

    pub fn update_substeps(&mut self, dt: f32, sub_steps: u32) {
        let sub_dt = dt / sub_steps as f32;
        for _ in 0..sub_steps {
            self.update(sub_dt);
        }
    }

    //update the enviroment
    pub fn update(&mut self, dt: f32) {
        for i in 0..self.joints.len() {
            {
                let &grav = &self.acceleration;
                let j = &mut self.joints[i];
                if self.allow_acceleration {
                    j.accelerate(grav);
                }

                if self.allow_move {
                    j.update(dt);
                }

                if self.allow_drag {
                    j.experience_drag();
                }

                if self.allow_bounce {
                    j.bounce(self.width as f32, self.height as f32);
                }

                if j.speed.abs() < self.stable{
                    j.speed = 0.0;
                }

                // todo: spacial partitioning fix
                for j in 0..self.joints.len() {
                    if i != j {
                        let mut v = self.joints[i];
                        let n = &mut self.joints[j];
                        
                        if self.allow_collide {
                            v.collide(n, dt);
                        }
                    }
                }

            }
        }

        for i in 0..self.springs.len() {
            let springs = &mut self.springs[i];
            let v = springs.update(&self.joints[springs.indices.0], &self.joints[springs.indices.1]);
            {
                let joint = &mut self.joints;
                joint[springs.indices.0].accelerate(v.0);
                joint[springs.indices.1].accelerate(v.1);
            }

        }
        
    }
    
}