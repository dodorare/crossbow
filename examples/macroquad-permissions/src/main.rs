use crossbow::crossbow_permissions::prelude::*;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    let skin = {
        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color_selected(Color::from_rgba(190, 190, 190, 255))
            .font_size(50)
            .build();

        Skin {
            editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };

    let window_skin = skin.clone();
    loop {
        clear_background(MAROON);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(0.0, 250.0), vec2(500.0, 500.0), |ui| {
            if ui.button(vec2(-15.0, 150.0), "Ask camera permission") {
                // request_permission(Permission::AndroidPermission(AndroidPermission::Camera))
                //     .unwrap();
                check_permission(Permission::AndroidPermission(AndroidPermission::Camera)).unwrap();
            }
            if ui.button(vec2(-15.0, 300.0), "Ask storage permission") {
                check_permission(Permission::AndroidPermission(
                    AndroidPermission::ReadExternalStorage,
                ))
                .unwrap();
            }
        });
        root_ui().pop_skin();
        next_frame().await;
    }
}
