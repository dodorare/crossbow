use macroquad::prelude::*;

#[macroquad::main("Macroquad 3D")]
async fn main() -> anyhow::Result<()> {
    #[cfg(not(target_os = "android"))]
    let image = get_assets_from_path();
    #[cfg(not(target_os = "android"))]
    let rust_logo = load_texture(&image).await?;

    #[cfg(target_os = "android")]
    let rust_logo = load_texture("images/rust.png").await?;

    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), Some(&rust_logo), WHITE);

        draw_cube(
            vec3(-5., 1., -2.),
            vec3(2., 2., 2.),
            Some(&rust_logo),
            WHITE,
        );
        draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), Some(&rust_logo), WHITE);
        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

// Workaround. Failed to get assets on windows from the macroquad .load_texture() method
// through the relative path to asset
#[cfg(not(target_os = "android"))]
fn get_assets_from_path() -> String {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = manifest_dir.parent().unwrap().parent().unwrap();
    let image_path = std::path::PathBuf::from("assets").join("images/rust.png");
    let image = assets_dir.join(image_path).to_str().unwrap().to_owned();
    image
}
