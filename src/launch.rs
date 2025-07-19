use std::path::PathBuf;

use crate::app::PartyConfig;
use crate::game::Game;
use crate::handler::*;
use crate::input::*;
use crate::launch::Game::{Executable, HandlerRef};
use crate::paths::*;
use crate::util::*;

pub fn launch_game(
    game: &Game,
    input_devices: &[DeviceInfo],
    instances: &Vec<Instance>,
    cfg: &PartyConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if let HandlerRef(h) = game {
        for instance in instances {
            create_profile(instance.profname.as_str())?;
            create_gamesave(instance.profname.as_str(), &h)?;
        }
        if h.symlink_dir {
            create_symlink_folder(&h)?;
        }
    }

    let cmd = launch_cmd(game, input_devices, instances, cfg)?;
    println!("\nCOMMAND:\n{}\n", cmd);

    if cfg.enable_kwin_script {
        let script = if instances.len() == 2 && cfg.vertical_two_player {
            "splitscreen_kwin_vertical.js"
        } else {
            "splitscreen_kwin.js"
        };

        kwin_dbus_start_script(PATH_RES.join(script))?;
    }

    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;

    if cfg.enable_kwin_script {
        kwin_dbus_unload_script()?;
    }

    remove_guest_profiles()?;

    Ok(())
}

pub fn launch_cmd(
    game: &Game,
    input_devices: &[DeviceInfo],
    instances: &Vec<Instance>,
    cfg: &PartyConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let home = PATH_HOME.display();
    let localshare = PATH_LOCAL_SHARE.display();
    let party = PATH_PARTY.display();
    let steam = PATH_STEAM.display();
    let res = PATH_RES.display();

    let mut gsc_lowres_warn = true;

    let gamedir = match game {
        Executable { .. } => "",
        HandlerRef(h) => match h.symlink_dir {
            true => &format!("{party}/gamesyms/{}", h.uid),
            false => &get_rootpath_handler(&h)?,
        },
    };

    let win = match game {
        Executable { path, .. } => path.extension().unwrap_or_default() == "exe",
        HandlerRef(h) => h.win,
    };

    let mut cmd = String::new();
    // Command: "gamescope [settings] -- bwrap [binds] [runtime] [exec] [args] & ..."
    cmd.push_str("export ");
    cmd.push_str("SDL_JOYSTICK_HIDAPI=0 ");
    cmd.push_str("ENABLE_GAMESCOPE_WSI=0 ");
    cmd.push_str("PROTON_DISABLE_HIDRAW=1 ");

    if cfg.force_sdl && !win {
        let mut path_sdl = "/ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0";
        if let HandlerRef(h) = game {
            if h.is32bit {
                path_sdl = "/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libSDL2-2.0.so.0";
            }
        };
        cmd.push_str(&format!("SDL_DYNAMIC_API=\"{steam}/{path_sdl}\" "));
    }
    if win {
        let protonpath = match cfg.proton_version.is_empty() {
            true => "GE-Proton",
            false => cfg.proton_version.as_str(),
        };
        cmd.push_str(&format!("PROTON_VERB=run WINEPREFIX={party}/pfx "));
        cmd.push_str(&format!("PROTONPATH={protonpath} "));

        if let HandlerRef(h) = game {
            if !h.dll_overrides.is_empty() {
                cmd.push_str("WINEDLLOVERRIDES=\"");
                for dll in &h.dll_overrides {
                    cmd.push_str(&format!("{dll},"));
                }
                cmd.push_str("=n,b\" ");
            }
            if h.coldclient {
                cmd.push_str("PROTON_DISABLE_LSTEAMCLIENT=1 ");
            }
        }
    }
    cmd.push_str("; ");

    let runtime = match win {
        true => &format!("{res}/umu-run"),
        false => {
            if let HandlerRef(h) = game {
                match h.runtime.as_str() {
                    "scout" => &format!("\"{steam}/ubuntu12_32/steam-runtime/run.sh\""),
                    "soldier" => &format!(
                        "\"{steam}/steamapps/common/SteamLinuxRuntime_soldier/_v2-entry-point\""
                    ),
                    _ => "",
                }
            } else {
                ""
            }
        }
    };

    let exec = match game {
        Executable { path, .. } => &path.to_string_lossy(),
        HandlerRef(h) => h.exec.as_str(),
    };

    if !PathBuf::from(gamedir).join(exec).exists() {
        return Err(format!("Executable not found: {gamedir}/{exec}").into());
    }

    if let HandlerRef(h) = game {
        if h.runtime == "scout" && !PATH_STEAM.join("ubuntu12_32/steam-runtime/run.sh").exists() {
            return Err("Steam Scout Runtime not found".into());
        } else if h.runtime == "soldier"
            && !PATH_STEAM
                .join("steamapps/common/SteamLinuxRuntime_soldier")
                .exists()
        {
            return Err("Steam Soldier Runtime not found".into());
        }
    }

    let (screen_width, screen_height) = get_screen_resolution();
    let scale_factor = cfg.render_scale as f32 / 100.0;
    let width = (screen_width as f32 * scale_factor) as u32;
    let height = (screen_height as f32 * scale_factor) as u32;

    cmd.push_str(&format!("cd \"{gamedir}\"; "));

    for (i, instance) in instances.iter().enumerate() {
        let path_prof = &format!("{party}/profiles/{}", instance.profname.as_str());
        let path_save = match game {
            Executable { .. } => "",
            HandlerRef(h) => &format!("{path_prof}/saves/{}", h.uid.as_str()),
        };

        let (gsc_width, gsc_height) =
            get_instance_resolution(instances.len(), i, width, height, cfg.vertical_two_player);

        if gsc_height < 600 && gsc_lowres_warn {
            msg(
                "Resolution warning",
                "Instance resolution is below 600p! The game may experience graphical issues or not run at all. Increase the resolution scale in settings if this happens.",
            );
            gsc_lowres_warn = false;
        }

        let gsc_sdl = match cfg.gamescope_sdl_backend {
            true => "--backend=sdl",
            false => "",
        };

        let gamescope = match cfg.kbm_support {
            true => &format!("{res}/gamescope"),
            false => "gamescope",
        };

        if cfg.enable_cpu_affinity {
            let affinity_cores: Vec<&str> = cfg.cpu_affinity_pattern.split(';').collect();
            if let Some(cores) = affinity_cores.get(i) {
                cmd.push_str(&format!("taskset -c {} ", cores));
            }
        }

        cmd.push_str(&format!(
            "{gamescope} -W {gsc_width} -H {gsc_height} {gsc_sdl} "
        ));

        if cfg.kbm_support {
            let mut instance_has_keyboard = false;
            let mut instance_has_mouse = false;
            let mut kbms = String::new();

            for d in &instance.devices {
                if input_devices[*d].device_type == DeviceType::Keyboard {
                    instance_has_keyboard = true;
                } else if input_devices[*d].device_type == DeviceType::Mouse {
                    instance_has_mouse = true;
                }
                if input_devices[*d].device_type == DeviceType::Keyboard
                    || input_devices[*d].device_type == DeviceType::Mouse
                {
                    kbms.push_str(&format!("{},", input_devices[*d].path));
                }
            }

            if instance_has_keyboard {
                cmd.push_str("--backend-disable-keyboard ");
            }
            if instance_has_mouse {
                cmd.push_str("--backend-disable-mouse ");
            }
            if !kbms.is_empty() {
                cmd.push_str(&format!("--libinput-hold-dev {} ", kbms));
            }
        }

        cmd.push_str(&format!("-- "));

        cmd.push_str(&format!(
            "bwrap --die-with-parent --dev-bind / / --tmpfs /tmp "
        ));

        // Bind player profile directories to the game's directories
        let mut binds = String::new();
        let mut args = String::new();

        // Mask out any gamepads that aren't this player's
        // Disable for now
        for (d, dev) in input_devices.iter().enumerate() {
            if !dev.enabled
                || (!instance.devices.contains(&d) && dev.device_type == DeviceType::Gamepad)
            {
                let path = &dev.path;
                binds.push_str(&format!("--bind /dev/null {path} "));
            }
        }

        if let HandlerRef(h) = game {
            let path_goldberg = h.path_goldberg.as_str();
            if !path_goldberg.is_empty() {
                binds.push_str(&format!(
                    "--bind \"{path_prof}/steam\" \"{gamedir}/{path_goldberg}/goldbergsave\" "
                ));
            }
            if h.win {
                let path_windata = format!("{party}/pfx/drive_c/users/steamuser/");
                if h.win_unique_appdata {
                    binds.push_str(&format!(
                        "--bind \"{path_save}/_AppData\" \"{path_windata}/AppData\" "
                    ));
                }
                if h.win_unique_documents {
                    binds.push_str(&format!(
                        "--bind \"{path_save}/_Documents\" \"{path_windata}/Documents\" "
                    ));
                }
            } else {
                if h.linux_unique_localshare {
                    binds.push_str(&format!("--bind \"{path_save}/_share\" \"{localshare}\" "));
                }
                if h.linux_unique_config {
                    binds.push_str(&format!(
                        "--bind \"{path_save}/_config\" \"{home}/.config\" "
                    ));
                }
            }
            for subdir in &h.game_unique_paths {
                binds.push_str(&format!(
                    "--bind \"{path_save}/{subdir}\" \"{gamedir}/{subdir}\" "
                ));
            }

            args = h
                .args
                .iter()
                .map(|arg| match arg.as_str() {
                    "$GAMEDIR" => format!(" \"{gamedir}\""),
                    "$PROFILE" => format!(" \"{}\"", instance.profname.as_str()),
                    "$WIDTH" => format!(" {gsc_width}"),
                    "$HEIGHT" => format!(" {gsc_height}"),
                    "$WIDTHXHEIGHT" => format!(" \"{gsc_width}x{gsc_height}\""),
                    _ => format!(" {arg}"),
                })
                .collect::<String>();
        }

        cmd.push_str(&format!("{binds} {runtime} \"{gamedir}/{exec}\"{args} "));

        if i < instances.len() - 1 {
            // Proton games need a ~5 second buffer in-between launches
            // TODO: investigate why this is
            match win {
                true => cmd.push_str("& sleep 6; "),
                false => cmd.push_str("& sleep 0.01; "),
            }
        }
    }

    Ok(cmd)
}
