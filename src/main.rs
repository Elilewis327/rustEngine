#![allow(non_snake_case)]
use glfw::*;
use std::mem;
use std::env;

pub mod gl_funcs;
use crate::gl_funcs::*;

fn main(){
    
    let (mut glfw, mut window, events) = init();

    gl::load_with(|f_name| window.get_proc_address(f_name) as *const _); 

    let shader_program =
        ShaderProgram::from_file_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

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
        
        draw(&shader_program);
    }

}


pub fn init() -> (glfw::Glfw, glfw::Window, std::sync::mpsc::Receiver<(f64, WindowEvent)>) {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    
    match env::consts::OS {
        "macos" => glfw.window_hint(WindowHint::OpenGlForwardCompat(true)),
        "linux" => {},
        _ => panic!("Get a real computer. (Unsupported OS)")
    }
 
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(800, 600, "Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    (glfw, window, events)
}



#[allow(unused_variables)]
fn draw (shader_program: &ShaderProgram){
    clear_color(0.2, 0.3, 0.3, 1.0); // set this to like the sky color or something
    clear();

    square(shader_program, -1.0, -1.0, 1.0, 1.0, 0.0, 0.2, 1.0, 1.0);
    
    unsafe { 
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
    }
}


/// function that accept
pub fn square(shader_program: &ShaderProgram, x: f32, y: f32, w: f32, z: f32, r: f32, g: f32, b: f32, a: f32) {
   
    let vertices: [Vertex; 4] = [ [x, y, 0.0], [x, z, 0.0], [w, z, 0.0], [w, y, 0.0]];

    change_draw_color(shader_program, &"ourColor", &r, &g, &b, &a);
    
    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make the vertex buffer");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&vertices),
        gl::STATIC_DRAW,
    );

    let ebo = Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(BufferType::ElementArray);
    
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        gl::STATIC_DRAW,
    );


    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        gl::EnableVertexAttribArray(0);
    }   
    //polygon_mode(PolygonMode::Line);
    
}

//consts
type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];


const VERT_SHADER: &str = "./vert.glsl";
const FRAG_SHADER: &str = "./frag.glsl";
