use dialog::DialogBox;
use std::error::Error;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use x11rb::connection::Connection;

use crate::paths::*;

pub fn msg(title: &str, contents: &str) {
    let _ = dialog::Message::new(contents).title(title).show();
}

pub fn get_screen_resolution() -> (u32, u32) {
    if let Ok(conn) = x11rb::connect(None) {
        let screen = &conn.0.setup().roots[0];
        println!(
            "Got screen resolution: {}x{}",
            screen.width_in_pixels, screen.height_in_pixels
        );
        return (
            screen.width_in_pixels as u32,
            screen.height_in_pixels as u32,
        );
    }
    // Fallback to a common resolution if detection fails
    println!("Failed to detect screen resolution, using fallback 1920x1080");
    (1920, 1080)
}

// Gets the resolution for a specific instance based on the number of instances
pub fn get_instance_resolution(
    playercount: usize,
    i: usize,
    basewidth: u32,
    baseheight: u32,
) -> (u32, u32) {
    let (w, h) = match playercount {
        1 => (basewidth, baseheight),
        2 => (basewidth, baseheight / 2),
        3 => {
            if i == 0 {
                (basewidth, baseheight / 2)
            } else {
                (basewidth / 2, baseheight / 2)
            }
        }
        4 => (basewidth / 2, baseheight / 2),
        // 5 => {
        //     if i < 2 {
        //         (basewidth / 2, baseheight / 2)
        //     } else {
        //         (basewidth / 3, baseheight / 2)
        //     }
        // }
        // 6 => (basewidth / 3, baseheight / 2),
        // 7 => {
        //     if i < 2 || i > 4 {
        //         (basewidth / 2, baseheight / 3)
        //     } else {
        //         (basewidth / 3, baseheight / 3)
        //     }
        // }
        // 8 => (basewidth / 2, baseheight / 4),
        _ => (basewidth, baseheight),
    };
    println!("Resolution for instance {}/{playercount}: {w}x{h}", i + 1);
    return (w, h);
}

pub fn create_proton_pfx(pfx: PathBuf) -> Result<(), Box<dyn Error>> {
    if pfx.exists() {
        println!("Prefix {} exists, continuing...", pfx.display());
        return Ok(());
    }

    let umu = PATH_RES.join("umu-run");
    let reg = PATH_RES.join("wine_disable_hidraw.reg");
    let hidrawpatch = format!(
        "WINEPREFIX=\"{}\" \"{}\" regedit \"{}\"",
        pfx.display(),
        umu.display(),
        reg.display()
    );

    println!("Disabling hidraw in the wine prefix.....");
    let err = std::process::Command::new("sh")
        .arg("-c")
        .arg(&hidrawpatch)
        .status()?;
    if !err.success() {
        return Err("Failed to disable hidraw in the wine prefix".into());
    }

    sleep(Duration::from_secs(5));

    println!("Done.");
    Ok(())
}

// Sends the splitscreen script to the active KWin session through DBus
pub fn kwin_dbus_start_script(file: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Loading script {}...", file.display());
    if !file.exists() {
        return Err("Script file doesn't exist!".into());
    }

    let conn = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )?;

    let _: i32 = proxy.call("loadScript", &(file.to_string_lossy(), "splitscreen"))?;
    println!("Script loaded. Starting...");
    let _: () = proxy.call("start", &())?;

    println!("KWin script started.");
    Ok(())
}

pub fn kwin_dbus_unload_script() -> Result<(), Box<dyn Error>> {
    println!("Unloading splitscreen script...");
    let conn = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )?;

    let _: bool = proxy.call("unloadScript", &("splitscreen"))?;

    println!("Script unloaded.");
    Ok(())
}
