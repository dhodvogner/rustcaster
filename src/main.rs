extern crate gl;
extern crate glutin;

use glutin::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use rustcaster::{init_lib, setup, present, get_output_buffer_pointer, key_up, key_down};

static WIDTH : i32 = 1024;
static HEIGHT : i32 = 512;

fn main() {
    init_lib();
    setup(WIDTH, HEIGHT);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rustcaster")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64));

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (2, 0)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create window");

    let gl_context = unsafe { 
        gl_context
            .make_current()
            .expect("Cannot make context current")
    };

    // Initialize gl before we call any OpenGL function
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                // Keeping this but the window is not resizable
                // If the window is resized, we need to resize the output buffer as well.
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                WindowEvent::KeyboardInput { input, .. } => {
                    println!("Kyeboard input: {:?}", input);
                    imitate_js_input(input);
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // TODO: calculate delta time
                    present(0.0);

                    // TODO: Get output buffer
                    let buffer_ptr = get_output_buffer_pointer();

                    // TODO: Draw output buffer?
                    // I need something like: gl::DrawPixels(1024, 512, gl::RGBA, gl::UNSIGNED_BYTE, buffer);
                }
                gl_context.swap_buffers().unwrap();
            },            
            _ => (),
        }
    });
}

// TODO: make an enum for the keys
fn imitate_js_input(input: KeyboardInput) {
    match input.virtual_keycode {
        Some(VirtualKeyCode::W) => {
            if input.state == ElementState::Pressed {
                key_down(87);
            } else {
                key_up(87);
            }
        },
        Some(VirtualKeyCode::A) => {
            if input.state == ElementState::Pressed {
                key_down(68);
            } else {
                key_up(68);
            }
        },
        Some(VirtualKeyCode::S) => {
            if input.state == ElementState::Pressed {
                key_down(83);
            } else {
                key_up(83);
            }
        },
        Some(VirtualKeyCode::D) => {
            if input.state == ElementState::Pressed {
                key_down(65);
            } else {
                key_up(65);
            }
        },
        _ => (),
    }
}