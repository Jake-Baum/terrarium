
use three_d::*;

fn main() {
    let window = Window::new(WindowSettings {
        title: "A nice triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .expect("failed to construct window");

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Srgba::RED,   // bottom right
        Srgba::GREEN, // bottom left
        Srgba::BLUE,  // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        model.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &model, &[]);

        FrameOutput::default()
    });
}
