#![allow(non_snake_case)]
use glfw::*;
use nalgebra_glm as glm;

pub mod engine;
use crate::engine::gl_funcs::*;

fn main(){
    
    let (mut glfw, mut window, events, shader_program) = engine::gl_funcs::init(VERT_SHADER, FRAG_SHADER);

    let mut s1 = Square::new(&shader_program, &[0.0, 0.0, 1.0, 1.0], &[1.0, 0.0, 0.0, 1.0]);

    window.make_current();
    window.set_key_polling(true);


    while !window.should_close(){
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
        
        draw(&mut glfw, &mut s1);
    }

}



#[allow(unused_variables)]
fn draw (glfw: &mut glfw::Glfw, s1: &mut Square){
    clear_color(0.2, 0.3, 0.3, 1.0); // set this to like the sky color or something
    clear();

    let mut v4 = glm::make_vec4::<f32>(&s1.get_pos());

    v4 = glm::rotate_vec4::<f32>(&v4, glfw.get_time() as f32, &glm::make_vec3::<f32>(&[1.0, 0.0, 0.0]));

    s1.set_pos(&[v4.x, v4.y, v4.w, v4.z]);

    s1.draw();
    
}

const VERT_SHADER: &str = "./vert.glsl";
const FRAG_SHADER: &str = "./frag.glsl";
