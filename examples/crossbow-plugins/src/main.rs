use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

#[cfg(target_os = "android")]
struct AppPlugins {
    billing: play_billing::PlayBillingPlugin,
    games_services: play_games_services::PlayGamesServicesPlugin,
    core: play_core::PlayCorePlugin,
}

#[macroquad::main("Macroquad UI")]
async fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "android")]
    let app_plugins = {
        let crossbow = crossbow::android::CrossbowInstance::new();

        let app_plugins = AppPlugins {
            billing: crossbow.get_plugin()?,
            games_services: crossbow.get_plugin()?,
            core: crossbow.get_plugin()?,
        };

        println!("Calling games_services.init()");
        app_plugins.games_services.init(true)?;
        println!("Calling billing.start_connection()");
        app_plugins.billing.start_connection()?;
        println!("Calling billing.start_connection()");
        app_plugins.core.check_update()?;

        app_plugins
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
            ui.label(vec2(15.0, 0.0), "Crossbow Plugins");
            #[cfg(target_os = "android")]
            ui.label(vec2(15.0, 50.0), &label);

            #[cfg(target_os = "android")]
            let btn_text = "Sign in";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 100.0), btn_text) {
                _btn_clicked = btn_text;
            }

            #[cfg(target_os = "android")]
            let btn_text = "Query SKU";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 150.0), btn_text) {
                _btn_clicked = btn_text;
            }

            #[cfg(target_os = "android")]
            let btn_text = "Purchase";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 200.0), btn_text) {
                _btn_clicked = btn_text;
            }

            #[cfg(target_os = "android")]
            let btn_text = "Query";
            #[cfg(target_os = "android")]
            if ui.button(vec2(-15.0, 250.0), btn_text) {
                _btn_clicked = btn_text;
            }
        });
        root_ui().pop_skin();

        #[cfg(target_os = "android")]
        match _btn_clicked {
            "Sign In" => {
                println!("Calling sign_in()");
                app_plugins.games_services.sign_in()?;
            }
            "Start Connection" => {}
            "Query SKU" => {
                println!("Calling query_sku_details(_)");
                app_plugins
                    .billing
                    .query_sku_details(&["crossbow_item_1", "item_2"], "inapp")?;
            }
            "Query" => {
                println!("Calling query_purchases(_)");
                app_plugins.billing.query_purchases("inapp")?;
            }
            "Purchase" => {
                println!("Calling purchase(_)");
                let res = app_plugins.billing.purchase("crossbow_item_1")?;
                println!("purchase: {:?}", res);
            }
            _ => {}
        }
        #[cfg(target_os = "android")]
        handle_signals(&mut label, &app_plugins).await?;

        next_frame().await;
    }
}

#[cfg(target_os = "android")]
async fn handle_signals(label: &mut String, app_plugins: &AppPlugins) -> anyhow::Result<()> {
    use crossbow::android::plugin::CrossbowPlugin;
    if let Ok(signal) = app_plugins.billing.get_receiver().try_recv() {
        handle_signal(label, signal)?;
    }
    if let Ok(signal) = app_plugins.games_services.get_receiver().try_recv() {
        handle_signal(label, signal)?;
    }
    if let Ok(signal) = app_plugins.core.get_receiver().try_recv() {
        handle_signal(label, signal)?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
fn handle_signal(
    label: &mut String,
    signal: crossbow::android::plugin::Signal,
) -> anyhow::Result<()> {
    println!("Signal: {:?}", signal);
    *label = format!(
        "{}:{}",
        signal.name,
        signal
            .args
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .concat()
    );

    match signal.name.as_str() {
        "query_purchases_response" => {
            let res = signal.args[0].clone().into_map().unwrap();
            println!("res: {:?}", res);
            println!("status: {:?}", res.get("status"));
            println!("purchases: {:?}", res.get("purchases"));
        }
        &_ => {}
    }
    Ok(())
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
