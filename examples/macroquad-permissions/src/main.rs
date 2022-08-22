use crossbow::Permission;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "android")]
    let crossbow = crossbow::android::CrossbowInstance::new();
    #[cfg(target_os = "android")]
    let admob: admob_android::AdMobPlugin = crossbow.get_plugin()?;

    let skin = get_skin();
    let mut label = "".to_owned();
    let window_skin = skin.clone();
    #[allow(unused_assignments)]
    let mut btn_clicked = "";

    loop {
        btn_clicked = "";
        clear_background(WHITE);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(0.0, 50.0), vec2(1000.0, 1000.0), |ui| {
            #[cfg(target_os = "android")]
            ui.label(vec2(15.0, 0.0), "AdMob");
            ui.label(vec2(15.0, 50.0), &label);

            let btn_text = "Camera permission";
            if ui.button(vec2(-15.0, 100.0), btn_text) {
                btn_clicked = btn_text;
            }
            let btn_text = "Mic permission";
            if ui.button(vec2(-15.0, 150.0), btn_text) {
                btn_clicked = btn_text;
            }
            #[cfg(target_os = "ios")]
            let btn_text = "Photos permission";
            #[cfg(target_os = "ios")]
            if ui.button(vec2(-15.0, 200.0), btn_text) {
                btn_clicked = btn_text;
            }
            #[cfg(target_os = "android")]
            let btn_text = "Show ad";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 250.0), btn_text) {
                btn_clicked = btn_text;
            }
        });
        root_ui().pop_skin();

        match btn_clicked {
            "Camera permission" => {
                let res = Permission::Camera.request_async().await?;
                label = format!("Camera {:?}", res);
            }
            "Mic permission" => {
                let res = Permission::Microphone.request_async().await?;
                label = format!("Microphone {:?}", res);
            }
            #[cfg(target_os = "ios")]
            "Photos permission" => {
                let res = Permission::Photos.request_async().await?;
                label = format!("Photos {:?}", res);
            }
            #[cfg(target_os = "android")]
            "Show ad" => {
                if !admob.is_initialized()? {
                    println!("Calling AdMob::initialize()");
                    admob.initialize(true, "G", false, true)?;
                }

                if admob.is_initialized()? && !admob.is_interstitial_loaded()? {
                    println!("Calling load_interstitial()");
                    admob.load_interstitial("ca-app-pub-3940256099942544/1033173712")?;
                }

                if admob.is_interstitial_loaded()? {
                    println!("Calling show_interstitial()");
                    admob.show_interstitial()?;
                }
            }
            _ => {}
        }

        #[cfg(target_os = "android")]
        use crossbow::android::plugin::CrossbowPlugin;
        #[cfg(target_os = "android")]
        if let Ok(signal) = admob.get_receiver().try_recv() {
            println!("Signal: {:?}", signal);
            label = format!(
                "{}:{}",
                signal.name,
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

fn get_skin() -> Skin {
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
}
