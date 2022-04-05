use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("UI showcase")]
async fn main() {
    let skin = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("ui_assets/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("ui_assets/window_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("ui_assets/button_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("ui_assets/button_hovered_background.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("ui_assets/button_clicked_background.png"),
                None,
            ))
            .font(include_bytes!("ui_assets/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .font(include_bytes!("ui_assets/HTOWERT.TTF"))
            .unwrap()
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

    let mut window_skin = skin.clone();
    // let action_font = load_ttf_font("ui_assets/HTOWERT.TTF").await.unwrap();
    loop {
        clear_background(BROWN);

        root_ui().group(hash!(), vec2(70.0, 100.0), |ui| {
            ui.label(None, "Window");

            if ui.button(None, "Skin") {
                window_skin = skin.clone();
            }
        });

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(250., 150.), vec2(300., 300.), |ui| {
            if ui.button(vec2(65.0, 15.0), "Play") {
                draw_text_ex("Let's play!", 100.0, 400.0, TextParams::default());
            }
            if ui.button(vec2(40.0, 75.0), "Options") {
                draw_text_ex("Some options", 100.0, 400.0, TextParams::default());
            }
            if ui.button(vec2(65.0, 195.0), "Quit") {
                draw_text_ex("Quit", 100.0, 400.0, TextParams::default());
            }
        });
        root_ui().pop_skin();

        next_frame().await;
    }
}
