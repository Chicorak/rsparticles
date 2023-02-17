//include rsparticle/joint.rs
mod enviroment;
use raylib::{prelude::*};

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(600, 600)
    .title("Hello, World")
    .build();

    let e = &mut enviroment::Enviroment::new(600, 600);

    let mut colors: Vec<Color> = Vec::new();

    e.add_joint_custom(300.0, 300.0, 10.0, 600.0, 0.0, 0.0, 0.1);
    e.add_joint_custom(500.0, 300.0, 10.0, 600.0, 0.0, 0.0, 0.1);
    e.add_joint_custom(500.0, 500.0, 10.0, 600.0, 0.0, 0.0, 0.1);
    e.add_joint_custom(300.0, 500.0, 10.0, 600.0, 0.0, 0.0, 0.1);
    for i in 0..4 {
        colors.push(Color::color_from_hsv(i as f32, 1.0, 1.0));
    }

    e.add_spring(1-1, 2-1, 100.0, 100.0);
    e.add_spring(2-1, 3-1, 100.0, 100.0);
    e.add_spring(3-1, 4-1, 100.0, 100.0);
    e.add_spring(4-1, 1-1, 100.0, 100.0);
    e.add_spring(1-1, 3-1, 100.0, 100.0);
    e.add_spring(2-1, 4-1, 100.0, 100.0);
    
    
    let mut selected_joint = e.get_joints().len();
    
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            e.add_joint(10.0, 20.0, 1.0, PI as f32/1.5, 100.0);
            colors.push(Color::color_from_hsv(e.get_joints().len() as f32 * 3.0, 1.0, 1.0));
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse = rl.get_mouse_position();
            {
                let index = e.get_joint_at(mouse.x, mouse.y);
                if index != e.get_joints().len() {
                    selected_joint = index;
                }
                if selected_joint != e.get_joints().len()
                    { e.get_joint(selected_joint as usize).move_to(mouse.x, mouse.y); }

            }
        }

        {
            e.update_substeps(rl.get_frame_time(), 4);

            let l = rl.get_frame_time().to_string();
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            d.draw_text(&("selected joint: ".to_owned() + selected_joint.to_string().as_str()), 0, 0, 20, Color::WHITE);
            d.draw_fps(0, 20);
            d.draw_text(l.as_str(), 0, 40, 20, Color::BLUE);
            d.draw_text(e.get_joints().len().to_string().as_str(), 0, 60, 20, Color::RED);

            
            for l in 0..e.get_joints().len() {
                // get js index
                let j = e.get_joint(l);
                d.draw_circle(j.x as i32, j.y as i32, j.size as f32, colors[l]);
            }

            for s in e.get_springs() {
                d.draw_line(s.position_1.0 as i32, s.position_1.1 as i32, s.position_2.0 as i32, s.position_2.1 as i32, Color::WHITE);
            }

            //draw number of each joint
            for i in 0..4 {
                let j = e.get_joint(i);
                d.draw_text(&i.to_string(), j.x as i32 - 10, j.y as i32 - 30, 20, Color::WHITE);
            }

        }
    }
}
