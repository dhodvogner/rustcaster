extern crate gl;
extern crate glutin;

mod desktop_utils;

use std::mem;
use std::ptr;
//use std::ffi::CString;
use glutin::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};
use gl::types::*;

use rustcaster::{init_lib, setup, present, get_output_buffer_pointer, key_up, key_down};
use desktop_utils::{compile_shader, link_program, VS_SRC, FS_SRC, VERTEX_DATA};

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
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create window");

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_context = unsafe { 
        gl_context
            .make_current()
            .expect("Cannot make context current")
    };

    // Initialize gl before we call any OpenGL function
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);
    let mut vbo = 0;
    let mut vao = 0;
    let mut texture = 0;

    println!("Program linked");

    //let pos_str = CString::new("position").unwrap();

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        println!("Data uploaded");

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Use shader program
        gl::UseProgram(program);

        //Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, pos_str.as_ptr());
        // gl::EnableVertexAttribArray(pos_attr as GLuint);
        // gl::VertexAttribPointer(
        //     pos_attr as GLuint,
        //     2,
        //     gl::FLOAT,
        //     gl::FALSE as GLboolean,
        //     0,
        //     ptr::null(),
        // );
    
        println!("Data specified");

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // Set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::Viewport(0, 0, WIDTH, HEIGHT);
    }

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
                    gl_context.window().request_redraw(); // TODO: Why it is super slooooowww??
                },
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        gl::DeleteProgram(program);
                        gl::DeleteShader(fs);
                        gl::DeleteShader(vs);
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteVertexArrays(1, &vao);
                        gl::DeleteTextures(1, &texture);
                    }

                    println!("Cleaned up");

                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    println!("Redraw requested");

                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    // TODO: calculate delta time
                    present(0.0);

                    // Generate texture from the output buffer
                    let buffer_ptr = get_output_buffer_pointer();
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA as i32,
                        WIDTH,
                        HEIGHT,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        buffer_ptr as *const _ as *const GLvoid,
                    );
                    gl::GenerateMipmap(gl::TEXTURE_2D);

                    gl::UseProgram(program);
                    gl::BindVertexArray(vao);
                    // Draw a triangle from the 3 vertices
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
                key_down(65);
            } else {
                key_up(65);
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
                key_down(68);
            } else {
                key_up(68);
            }
        },
        _ => (),
    }
}