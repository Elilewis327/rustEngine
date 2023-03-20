#![allow(non_snake_case)]
use glfw::*;
use std::mem;

pub mod gl_funcs;
use crate::gl_funcs::*;


fn main(){
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(800, 600, "Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    triangle(&mut window);

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
        
        draw(&mut window);

        // call gl get error probs
    }

}

fn draw (window: &mut glfw::Window){
    clear();
    unsafe { 
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
    }
}

fn triangle (window: &mut glfw::Window) {
    gl::load_with(|f_name| window.get_proc_address(f_name) as *const _); 

    clear_color(0.2, 0.3, 0.3, 1.0);
    
    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make the vertex buffer");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
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

    let shader_program =
        ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    
    shader_program.use_program();
    
    polygon_mode(PolygonMode::Line);

}

//consts
type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] =
  [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];


const VERT_SHADER: &str = r#"#version 330 core
            layout (location = 0) in vec3 pos;
            void main() {
                gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            }
            "#;

const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;
        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
        "#;


