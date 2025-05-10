mod app;
mod handler;
mod input;
mod launch;
mod paths;
mod util;

use crate::app::*;
use crate::paths::*;
use crate::util::*;
use dialog::DialogBox;

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
        let _ = dialog::Message::new(
            "UMU Launcher not found in resources folder. PartyDeck uses UMU to launch Windows games with Proton. Click OK to automatically download from the internet.",
        )
        .title("Download UMU")
        .show()
        .unwrap();
        update_umu_launcher().unwrap();
    }
    if !PATH_RES.join("goldberg_linux").exists() || !PATH_RES.join("goldberg_win").exists() {
        let _ = dialog::Message::new(
            "Goldberg Steam Emu not found in resources folder. PartyDeck uses Goldberg for LAN play. Click OK to automatically download from the internet.",
        )
        .title("Download Goldberg")
        .show()
        .unwrap();
        update_goldberg_emu().unwrap();
    }

    println!("\n[PARTYDECK] started\n");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([720.0, 360.0])
            .with_min_inner_size([640.0, 360.0])
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
            cc.egui_ctx.set_zoom_factor(1.3);
            Ok(Box::<PartyApp>::default())
        }),
    )
}
