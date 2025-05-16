mod app;
mod game;
mod handler;
mod input;
mod launch;
mod paths;
mod util;

use crate::app::*;
use crate::paths::*;
use crate::util::*;

fn main() -> eframe::Result {
    std::fs::create_dir_all(PATH_PARTY.join("gamesyms"))
        .expect("Failed to create gamesyms directory");
    std::fs::create_dir_all(PATH_PARTY.join("handlers"))
        .expect("Failed to create handlers directory");
    std::fs::create_dir_all(PATH_PARTY.join("profiles"))
        .expect("Failed to create profiles directory");

    remove_guest_profiles().unwrap();

    if PATH_PARTY.join("tmp").exists() {
        std::fs::remove_dir_all(PATH_PARTY.join("tmp")).unwrap();
    }
    if !PATH_RES.join("umu-run").exists() {
        msg(
            "Downloading Dependencies",
            "UMU Launcher not found in resources folder. PartyDeck uses UMU to launch Windows games with Proton. Click OK to automatically download from the internet.",
        );
        if let Err(e) = update_umu_launcher() {
            println!("Failed to download UMU Launcher: {}", e);
            msg("Error", &format!("Failed to download UMU Launcher: {}", e));
            return Ok(());
        }
    }
    if !PATH_RES.join("goldberg_linux").exists() || !PATH_RES.join("goldberg_win").exists() {
        msg(
            "Downloading Dependencies",
            "Goldberg Steam Emu not found in resources folder. PartyDeck uses Goldberg for LAN play. Click OK to automatically download from the internet.",
        );
        if let Err(e) = update_goldberg_emu() {
            println!("Failed to download Goldberg: {}", e);
            msg("Error", &format!("Failed to download Goldberg: {}", e));
            return Ok(());
        }
    }

    println!("\n[PARTYDECK] started\n");

    let fullscreen = std::env::args().any(|arg| arg == "--fullscreen");

    let (_, scrheight) = get_screen_resolution();

    let scale = match fullscreen {
        true => scrheight as f32 / 560.0,
        false => 1.3,
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1080.0, 540.0])
            .with_min_inner_size([640.0, 360.0])
            .with_fullscreen(fullscreen)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../res/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "PartyDeck",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_zoom_factor(scale);
            Ok(Box::<PartyApp>::default())
        }),
    )
}
