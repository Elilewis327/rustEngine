use gl;
use gl::types::*;

// safe to unsafe function bindings
// mostly stolen 
// from https://rust-tutorials.github.io/learn-opengl/basics/002-triangle-cleanup.html

///clear color
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { gl::ClearColor(r, g, b, a) }
}

///clear the screen
pub fn clear(){
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
}

/// vertex arrays
pub struct VertexArray (pub GLuint);
impl VertexArray {
    pub fn new() -> Option<Self>{
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }
    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self.0)}
    }

    pub fn clear_binding(&self) {
        unsafe {gl::BindVertexArray(0);}
    }
}

/// vertex buffers
pub enum BufferType {
  Array = gl::ARRAY_BUFFER as isize,
  ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
  pub fn new() -> Option<Self> {
    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); }
    if vbo != 0 {
      Some(Self(vbo))
    } else {
      None
    }
  }

  pub fn bind(&self, ty: BufferType) {
    unsafe { gl::BindBuffer(ty as GLenum, self.0) }
  }

  pub fn clear_binding(ty: BufferType) {
    unsafe { gl::BindBuffer(ty as GLenum, 0) }
  }
}

/// Places a slice of data into a previously-bound buffer.
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
  unsafe {
    gl::BufferData(
      ty as GLenum,
      data.len().try_into().unwrap(),
      data.as_ptr().cast(),
      usage,
    );
  }
}


pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

//TODO! Read shader from file
pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    pub fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    /// Compiles the shader based on the current source.
    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) };
    }
    
    /// Checks if the last compile was successful or not.
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(gl::TRUE)
    }
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
          gl::GetShaderInfoLog(
            self.0,
            v.capacity().try_into().unwrap(),
            &mut len_written,
            v.as_mut_ptr().cast(),
          );
          v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }
    
    /// Note: This _does not_ immediately delete the shader. It only marks it for
    /// deletion. If the shader has been previously attached to a program then the
    /// shader will stay allocated until it's unattached from that program.
    /// this can be called pretty much anywhere
    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }

    /// Takes a shader type and source string and produces either the compiled
    /// shader or an error message.
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty)
            .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
          Ok(id)
        } else {
          let out = id.info_log();
          id.delete();
          Err(out)
        }
    }
}

pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {

    /// creates a new shader program
    pub fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 {
          Some(Self(prog))
        } else {
          None
        }
    }

  /// Attaches a shader object to this program object.
  pub fn attach_shader(&self, shader: &Shader) {
    unsafe { gl::AttachShader(self.0, shader.0) };
  }

  /// Links the various attached, compiled shader objects into a usable program.
  pub fn link_program(&self) {
    unsafe { gl::LinkProgram(self.0) };
  }

  /// Checks if the last linking operation was successful.
  pub fn link_success(&self) -> bool {
    let mut success = 0;
    unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
    success == i32::from(gl::TRUE)
  }

  /// Gets the log data for this program.
  ///
  /// This is usually used to check the message when a program failed to link.
  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      gl::GetProgramInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  /// Sets the program as the program to use when drawing.
  pub fn use_program(&self) {
    unsafe { gl::UseProgram(self.0) };
  }

  /// Marks the program for deletion.
  ///
  /// Note: This _does not_ immediately delete the program. If the program is
  /// currently in use it won't be deleted until it's not the active program.
  /// When a program is finally deleted and attached shaders are unattached.
  pub fn delete(self) {
    unsafe { gl::DeleteProgram(self.0) };
  }

  /// Takes a vertex shader source string and a fragment shader source string
  /// and either gets you a working program object or gets you an error message.
  ///
  /// This is the preferred way to create a simple shader program in the common
  /// case. It's just less error prone than doing all the steps yourself.
  pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
    let p =
      Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
    let v = Shader::from_source(ShaderType::Vertex, vert)
      .map_err(|e| format!("Vertex Compile Error: {}", e))?;
    let f = Shader::from_source(ShaderType::Fragment, frag)
      .map_err(|e| format!("Fragment Compile Error: {}", e))?;
    p.attach_shader(&v);
    p.attach_shader(&f);
    p.link_program();
    v.delete();
    f.delete();
    if p.link_success() {
      Ok(p)
    } else {
      let out = format!("Program Link Error: {}", p.info_log());
      p.delete();
      Err(out)
    }
  }
}

pub enum PolygonMode {
  /// Just show the points.
  Point = gl::POINT as isize,
  /// Just show the lines.
  Line = gl::LINE as isize,
  /// Fill in the polygons.
  Fill = gl::FILL as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
  unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as GLenum) };
}