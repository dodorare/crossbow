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

    let (_, vm) = create_java_vm().unwrap();
    let jnienv = vm.attach_current_thread_as_daemon().unwrap();

    let admob_singleton =
        plugin::get_jni_singleton("AdMob").expect("Crossbow Error: AdMob is not registered");
    let admob =
        crossbow_admob_android::AdMobPlugin::from_jnienv(admob_singleton.clone(), jnienv).unwrap();

    let mut label = "Signal: ".to_owned();
    let window_skin = skin.clone();
    loop {
        clear_background(WHITE);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(0.0, 250.0), vec2(500.0, 500.0), |ui| {
            ui.label(vec2(15.0, 0.0), "AdMob");
            ui.label(vec2(15.0, 50.0), &label);
            if ui.button(vec2(-15.0, 150.0), "Ask camera permission") {
                request_permission(AndroidPermission::Camera).unwrap();
            }
            if ui.button(vec2(-15.0, 300.0), "Ask storage permission") {
                request_permission(AndroidPermission::ReadExternalStorage).unwrap();
            }
            if ui.button(vec2(-15.0, 450.0), "Show ad") {
                if !admob.get_is_initialized().unwrap() {
                    println!("calling initialize()");
                    admob.initialize(true, "G", false, true).unwrap();
                }

                if admob.get_is_initialized().unwrap()
                    && !admob.get_is_interstitial_loaded().unwrap()
                {
                    println!("calling load_interstitial()");
                    admob
                        .load_interstitial("ca-app-pub-3940256099942544/1033173712")
                        .unwrap();
                }

                if admob.get_is_interstitial_loaded().unwrap() {
                    println!("calling show_interstitial()");
                    admob.show_interstitial().unwrap();
                }

                // if admob.get_is_initialized().unwrap() && !admob.get_is_banner_loaded().unwrap() {
                //     println!("calling load_banner()");
                //     admob
                //         .load_banner(
                //             "ca-app-pub-3940256099942544/6300978111",
                //             0,
                //             crossbow_admob_android::BannerSize::FullBanner,
                //             true,
                //             true,
                //         )
                //         .unwrap();
                // }

                // if admob.get_is_banner_loaded().unwrap() {
                //     println!("calling show_banner()");
                //     admob.show_banner().unwrap();
                // }
            }
        });
        root_ui().pop_skin();

        if let Ok(signal) = admob_singleton.get_receiver().try_recv() {
            println!("signal: {:?}", signal);
            label = format!(
                "{}:{}",
                signal.signal_name,
                signal
                    .args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .concat()
            );
        }

        next_frame().await;
    }
}
