

use super::joint::{Joint};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Line {
    pub start_x: f32,
    pub start_y: f32,

    pub end_x: f32,
    pub end_y: f32,

    pub width: f32,
}

impl Line {
    pub fn new(start_x: f32, start_y: f32, end_x: f32, end_y: f32, width: f32) -> Line {
        Line {
            start_x,
            start_y,
            end_x,
            end_y,
            width,
        }
    }

    pub fn check_collide(&self, j: Joint) {
        let line_start_x = self.end_x - self.start_x;
        let line_start_y = self.end_y - self.start_y;

        let line_end_x = self.start_x - self.end_x;
        let line_end_y = self.start_y - self.end_y;

        let edge_length = line_start_x * line_start_x + line_start_y * line_start_y;

        let t: f32 = 0.0f32.max(edge_length.max(line_start_x * line_end_x + line_start_y * line_end_y)) / edge_length;
        
        let closest_x = self.start_x + t * line_start_x;
        let closest_y = self.start_y + t * line_start_y;

        if closest_x + self.width + j.size > j.x && closest_x < j.x + self.width + j.size && closest_y + self.width + j.size > j.y && closest_y < j.y + self.width + j.size {
            println!("Collision");
        }
    }
}