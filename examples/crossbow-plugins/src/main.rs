use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "android")]
    let (play_games, play_billing) = {
        let crossbow = crossbow::android::CrossbowInstance::new();

        let play_billing: play_billing::PlayBillingPlugin = crossbow.get_plugin()?;
        let play_games: play_games_services::PlayGamesServicesPlugin = crossbow.get_plugin()?;

        println!("Calling init()");
        play_games.init(true)?;

        (play_games, play_billing)
    };

    let skin = get_skin();
    let window_skin = skin.clone();
    #[cfg(target_os = "android")]
    let mut label = "-".to_owned();
    let mut _btn_clicked = "";

    loop {
        _btn_clicked = "";
        clear_background(WHITE);

        root_ui().push_skin(&window_skin);
        root_ui().window(hash!(), vec2(0.0, 50.0), vec2(1000.0, 1000.0), |ui| {
            #[cfg(target_os = "android")]
            ui.label(vec2(15.0, 0.0), "AdMob");

            ui.label(vec2(15.0, 0.0), "Play Games");
            #[cfg(target_os = "android")]
            ui.label(vec2(15.0, 50.0), &label);

            #[cfg(target_os = "android")]
            let btn_text = "Start Connection";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 100.0), btn_text) {
                _btn_clicked = btn_text;
            }

            #[cfg(target_os = "android")]
            let btn_text = "Purchase";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 150.0), btn_text) {
                _btn_clicked = btn_text;
            }
        });
        root_ui().pop_skin();

        #[cfg(target_os = "android")]
        match _btn_clicked {
            "Sign In" => {
                println!("Calling sign_in()");
                play_games.sign_in()?;
            }
            "Start Connection" => {
                println!("Calling start_connection()");
                play_billing.start_connection()?;
            }
            "Purchase" => {
                println!("Calling purchase(_)");
                play_billing.purchase("id")?;
            }
            _ => {}
        }

        #[cfg(target_os = "android")]
        use crossbow::android::plugin::CrossbowPlugin;
        #[cfg(target_os = "android")]
        if let Ok(signal) = play_billing.get_receiver().try_recv() {
            println!("Signal: {:?}", signal);
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
        #[cfg(target_os = "android")]
        if let Ok(signal) = play_games.get_receiver().try_recv() {
            println!("Signal: {:?}", signal);
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
