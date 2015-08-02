use gl;
use gl::types::GLint;
use noise::{Seed, Brownian1, perlin1};
use std;
use std::mem;
use sdl2;
use sdl2::event;
use sdl2::event::Event;
use sdl2::video;
use stopwatch::TimerSet;
use yaglw::gl_context::GLContext;
use yaglw::shader::Shader;
use yaglw::texture::{BufferTexture, TextureUnit};
use yaglw::vertex_buffer::{ArrayHandle};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub fn main() {
  let timers = TimerSet::new();

  let mut sdl = sdl2::init().everything().unwrap();
  let window = make_window(&sdl);

  let _sdl_gl_context = window.gl_create_context().unwrap();

  // Load the OpenGL function pointers.
  gl::load_with(|s| unsafe {
    mem::transmute(video::gl_get_proc_address(s))
  });

  let mut gl = unsafe {
    GLContext::new()
  };
  let gl = &mut gl;

  match gl.get_error() {
    gl::NO_ERROR => {},
    err => {
      println!("OpenGL error 0x{:x} in setup", err);
      return;
    },
  }

  let mut shader = make_shader(gl);
  shader.use_shader(gl);

  let heightmap = make_heightmap(gl);

  {
    let mut bind = |name, id| {
      let unit: TextureUnit = Default::default();
      unsafe {
        gl::ActiveTexture(unit.gl_id());
        gl::BindTexture(gl::TEXTURE_BUFFER, id);
      }
      let loc = shader.get_uniform_location(name);
      unsafe {
        gl::Uniform1i(loc, unit.glsl_id as GLint);
      }
    };

    bind("heightmap", heightmap.buffer.byte_buffer.handle.gl_id);
  }

  let empty_vao = ArrayHandle::new(gl);

  unsafe {
    gl::BindVertexArray(empty_vao.gl_id);
  }

  let mut event_pump = sdl.event_pump();

  while process_events(&mut event_pump) {
    timers.time("draw", || {
      gl.clear_buffer();
      unsafe {
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
      }
      // swap buffers
      window.gl_swap_window();
    });

    std::thread::sleep_ms(10);
  }

  timers.print();
}

fn make_shader<'a, 'b:'a>(
  gl: &'a GLContext,
) -> Shader<'b> {
  let vertex_shader: String = format!("
    #version 330 core

    out vec4 world_pos;

    void main() {{
      if (gl_VertexID == 0) {{
        world_pos = vec4(-1, 1, 0, 1);
      }} else if (gl_VertexID == 1) {{
        world_pos = vec4(-1, -1, 0, 1);
      }} else if (gl_VertexID == 2) {{
        world_pos = vec4(1, 1, 0, 1);
      }} else {{
        world_pos = vec4(1, -1, 0, 1);
      }}
      gl_Position = world_pos;
    }}
  ");

  let fragment_shader: String = format!("
    #version 330 core

    uniform samplerBuffer heightmap;

    in vec4 world_pos;

    layout(location=0) out vec4 frag_color;

    void main() {{
      int x = int(round(gl_FragCoord.x));
      float h = texelFetch(heightmap, x).r;
      if (world_pos.y <= h) {{
        frag_color = vec4(1, 0, 0, 1);
      }} else {{
        frag_color = vec4(0, 0, 0, 1);
      }}
    }}
  ");

  let components = vec!(
    (gl::VERTEX_SHADER, vertex_shader),
    (gl::FRAGMENT_SHADER, fragment_shader),
  );

  Shader::new(gl, components.into_iter())
}

fn make_window(sdl: &sdl2::Sdl) -> video::Window {
  video::gl_attr::set_context_profile(video::GLProfile::Core);
  video::gl_attr::set_context_version(3, 3);

  // Open the window as fullscreen at the current resolution.
  let mut window =
    video::WindowBuilder::new(
      &sdl,
      "Hello, Mandelbrot",
      WINDOW_WIDTH, WINDOW_HEIGHT,
    );

  let window = window.position_centered();
  window.opengl();

  window.build().unwrap()
}

fn make_heightmap<'a, 'b:'a>(
  gl: &'a mut GLContext,
) -> BufferTexture<'b, f32> {
  let mut ram_heightmap = [0.0; WINDOW_WIDTH as usize];

  let noise = Brownian1::new(perlin1, 1).frequency(1.0 / WINDOW_WIDTH as f32);
  let seed = Seed::new(0);

  for i in 0..WINDOW_WIDTH as usize {
    let h = noise.apply(&seed, &[i as f32]);
    ram_heightmap[i] = h;
  }

  let mut vram_heightmap = BufferTexture::new(gl, gl::R32F, WINDOW_WIDTH as usize);
  vram_heightmap.buffer.push(gl, &ram_heightmap);
  vram_heightmap
}

fn process_events<'a>(
  event_pump: &mut event::EventPump,
) -> bool {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit {..} => {
        return false;
      },
      Event::AppTerminating {..} => {
        return false;
      },
      _ => {},
    }
  }

  true
}
