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

    let p = s1.get_raw_pos();
    let np = [ p[0][0], p[0][1], p[0][2], 1.0, p[1][0], p[1][1], p[1][2], 1.0, p[2][0], p[2][1], p[2][2], 1.0, p[3][0], p[3][1], p[3][2], 1.0 ];
    let mut m = glm::Mat4::from_row_slice(&np); 

    m = glm::rotate::<f32>(&m, 0.0 as f32, &glm::vec3::<f32>(0.0, 0.0, 1.0));
    
    let h = m.as_slice();
    let fin: RawVec4 = [[h[0], h[1], h[2]], [h[4], h[5], h[6]], [h[8], h[9], h[10]], [h[12], h[13], h[14]] ]; 

    s1.set_raw_pos(&fin);

    s1.draw();
    
}

const VERT_SHADER: &str = "./vert.glsl";
const FRAG_SHADER: &str = "./frag.glsl";
