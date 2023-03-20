#![allow(non_snake_case)]
use glfw::*;

pub mod engine;
use crate::engine::gl_funcs::*;

fn main(){
    
    let (mut glfw, mut window, events, shader_program) = engine::gl_funcs::init(VERT_SHADER, FRAG_SHADER);



    let pos: Vec4 = [0.0, 0.0, 1.0, 1.0];
    let color: Vec4 = [1.0, 0.0, 0.0, 1.0];

    let mut s1 = square::new(&shader_program, pos, color);

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
        
        draw(&mut s1);
    }

}



#[allow(unused_variables)]
fn draw (s1: &mut square){
    clear_color(0.2, 0.3, 0.3, 1.0); // set this to like the sky color or something
    clear();
    s1.pos[0] += 0.001;
    s1.pos[1] += 0.001;
    s1.pos[2] += 0.001;
    s1.pos[3] += 0.001;

    s1.draw();
    
}

const VERT_SHADER: &str = "./vert.glsl";
const FRAG_SHADER: &str = "./frag.glsl";
