use crossbow::crossbow_android::*;
use crossbow::crossbow_android::{permission::*, types::*};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    crossbow::crossbow_android::init();

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
        clear_background(WHITE);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(0.0, 250.0), vec2(500.0, 500.0), |ui| {
            if ui.button(vec2(-15.0, 150.0), "Ask camera permission") {
                request_permission(AndroidPermission::Camera).unwrap();
            }
            if ui.button(vec2(-15.0, 300.0), "Ask storage permission") {
                request_permission(AndroidPermission::ReadExternalStorage).unwrap();
            }
            if ui.button(vec2(-15.0, 450.0), "Show ad") {
                let jni_singleton_guard = crossbow_plugin::get_jni_singletons();
                let admob = jni_singleton_guard
                    .get("AdMob")
                    .expect("Crossbow Error: AdMob is not registered");
                // println!("Crossbow AdMob Methods: {:?}", admob.get_methods());

                let (_, vm) = create_java_vm().unwrap();
                let jnienv = vm.attach_current_thread().unwrap();

                let g_str = jnienv.new_string("G".to_string()).unwrap();
                admob
                    .call_method(
                        &jnienv,
                        "initialize",
                        &[true.into(), g_str.into(), false.into(), true.into()],
                    )
                    .unwrap();

                let ad_id = jnienv
                    .new_string("ca-app-pub-3940256099942544/1033173712".to_string())
                    .unwrap();
                admob
                    .call_method(&jnienv, "load_interstitial", &[ad_id.into()])
                    .unwrap();

                admob
                    .call_method(&jnienv, "show_interstitial", &[])
                    .unwrap();
            }
        });
        root_ui().pop_skin();
        next_frame().await;
    }
}
