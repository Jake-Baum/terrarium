use std::time;

use winit::{
    event::*,
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    window::WindowBuilder,
};

use crate::{
    model::ModelVertex,
    state::State,
};

mod camera;
mod instance;
mod model;
mod state;
mod texture;

const VERTICES: &[ModelVertex] = &[
    ModelVertex {
        position: [-1.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 0.0, 1.0],
    }, // A
    ModelVertex {
        position: [1.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 0.0, 1.0],
    }, // B
    ModelVertex {
        position: [0.0, 2.0, 0.0],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 0.0, 1.0],
    }, // C
];

const INDICES: &[u32] = &[0, 1, 2];

#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Terrarium")
        .build(&event_loop)
        .expect("failed to build window");

    let mut state = State::new(window).await;
    let _last_render_time = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if state.mouse_pressed {
                    state.camera_controller.process_mouse(delta.0, delta.1)
                }
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() && !state.input(event) => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
