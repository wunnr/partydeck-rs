use std::path::PathBuf;

use crate::app::PartyConfig;
use crate::handler::*;
use crate::input::*;
use crate::paths::*;
use crate::util::{get_instance_resolution, get_rootpath_handler, get_screen_resolution, msg};

pub fn launch_from_handler(
    h: &Handler,
    all_pads: &Vec<Gamepad>,
    players: &Vec<Player>,
    cfg: &PartyConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let home = PATH_HOME.display();
    let localshare = PATH_LOCAL_SHARE.display();
    let party = PATH_PARTY.display();
    let steam = PATH_STEAM.display();
    let res = PATH_RES.display();

    let mut res_warn = true;

    let gamedir = match h.symlink_dir {
        true => format!("{party}/gamesyms/{}", h.uid),
        false => get_rootpath_handler(&h)?,
    };

    let mut cmd = String::new();
    // Command: "gamescope [settings] -- bwrap [binds] [runtime] [exec] [args] & ..."
    cmd.push_str("export ");
    cmd.push_str("SDL_JOYSTICK_HIDAPI=0 ");
    cmd.push_str("ENABLE_GAMESCOPE_WSI=0 ");
    cmd.push_str("PROTON_DISABLE_HIDRAW=1 ");

    if cfg.force_sdl && !h.win {
        let path_sdl = match h.is32bit {
            true => "/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu/libSDL2-2.0.so.0",
            false => "/ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0",
        };
        cmd.push_str(&format!("SDL_DYNAMIC_API=\"{steam}/{path_sdl}\" "));
    }
    if h.win {
        cmd.push_str(&format!("PROTON_VERB=run WINEPREFIX={party}/pfx "));
        let protonpath = match cfg.proton_version.is_empty() {
            true => "GE-Proton",
            false => cfg.proton_version.as_str(),
        };
        cmd.push_str(&format!("PROTONPATH={protonpath} "));
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
    cmd.push_str("; ");

    let exec = &h.exec.as_str();
    let runtime = match h.win {
        true => &format!("{res}/umu-run"),
        false => match h.runtime.as_str() {
            "scout" => &format!("\"{steam}/ubuntu12_32/steam-runtime/run.sh\""),
            "soldier" => {
                &format!("\"{steam}/steamapps/common/SteamLinuxRuntime_soldier/_v2-entry-point\"")
            }
            _ => "",
        },
    };

    if !PathBuf::from(gamedir.clone()).join(exec).exists() {
        return Err(format!("Executable ({exec}) not found").into());
    }

    if h.runtime == "scout" && !PATH_STEAM.join("ubuntu12_32/steam-runtime/run.sh").exists() {
        return Err("Steam Scout Runtime not found".into());
    } else if h.runtime == "soldier"
        && !PATH_STEAM
            .join("steamapps/common/SteamLinuxRuntime_soldier")
            .exists()
    {
        return Err("Steam Soldier Runtime not found".into());
    }

    let (screen_width, screen_height) = get_screen_resolution();
    let scale_factor = cfg.render_scale as f32 / 100.0;
    let width = (screen_width as f32 * scale_factor) as u32;
    let height = (screen_height as f32 * scale_factor) as u32;

    cmd.push_str(&format!("cd \"{gamedir}\"; "));
    for (i, p) in players.iter().enumerate() {
        let path_prof = &format!("{party}/profiles/{}", p.profname.as_str());
        let path_save = &format!("{path_prof}/saves/{}", h.uid.as_str());

        let (gsc_width, gsc_height) = get_instance_resolution(players.len(), i, width, height);

        if gsc_height < 600 && res_warn {
            msg(
                "Resolution warning",
                "Instance resolution is below 600p! The game may experience graphical issues or not run at all. Increase the resolution scale in settings if this happens.",
            );
            res_warn = false;
        }

        let gsc_sdl = match cfg.gamescope_sdl_backend {
            true => "--backend=sdl",
            false => "",
        };

        cmd.push_str(&format!(
            "gamescope -W {gsc_width} -H {gsc_height} {gsc_sdl} -- "
        ));
        cmd.push_str(&format!(
            "bwrap --die-with-parent --dev-bind / / --tmpfs /tmp "
        ));

        // Bind player profile directories to the game's directories
        let mut binds = String::new();

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
        // Mask out any gamepads that aren't this player's
        for (i, pad) in all_pads.iter().enumerate() {
            if p.pad_index == i {
                continue;
            } else {
                let path = pad.path();
                binds.push_str(&format!("--bind /dev/null {path} "))
            }
        }

        let args = h
            .args
            .iter()
            .map(|arg| match arg.as_str() {
                "$GAMEDIR" => format!(" \"{gamedir}\""),
                "$PROFILE" => format!(" \"{}\"", p.profname.as_str()),
                "$WIDTH" => format!(" {gsc_width}"),
                "$HEIGHT" => format!(" {gsc_height}"),
                "$WIDTHXHEIGHT" => format!(" \"{gsc_width}x{gsc_height}\""),
                _ => format!(" {arg}"),
            })
            .collect::<String>();

        cmd.push_str(&format!("{binds} {runtime} \"{gamedir}/{exec}\"{args} "));

        if i < players.len() - 1 {
            // Proton games need a ~5 second buffer in-between launches
            // TODO: investigate why this is
            if h.win {
                cmd.push_str("& sleep 6; ");
            } else {
                cmd.push_str("& sleep 0.01; ");
            }
        }
    }

    Ok(cmd)
}

pub fn launch_executable(
    exec_path: &PathBuf,
    all_pads: &Vec<Gamepad>,
    players: &Vec<Player>,
    cfg: &PartyConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let exec = exec_path.to_string_lossy();

    if !exec_path.exists() {
        return Err(format!("Executable ({exec}) not found").into());
    }

    let party = PATH_PARTY.display();
    let res = PATH_RES.display();

    let mut res_warn = true;

    let win = if exec_path.extension().unwrap_or_default() == "exe" {
        true
    } else {
        false
    };

    let runtime = match win {
        true => &format!("{res}/umu-run"),
        false => "",
    };

    let gamedir = exec_path.parent().unwrap().to_string_lossy();

    let mut cmd = String::new();
    // Command: "gamescope [settings] -- bwrap [binds] [runtime] [exec] [args] & ..."
    cmd.push_str("export ");
    cmd.push_str("SDL_JOYSTICK_HIDAPI=0 ");
    cmd.push_str("ENABLE_GAMESCOPE_WSI=0 ");

    if win {
        cmd.push_str(&format!("PROTON_VERB=run WINEPREFIX={party}/pfx "));
        let protonpath = match cfg.proton_version.is_empty() {
            true => "GE-Proton",
            false => cfg.proton_version.as_str(),
        };
        cmd.push_str(&format!("PROTONPATH={protonpath} "));
    }
    cmd.push_str("; ");

    let (screen_width, screen_height) = get_screen_resolution();
    let scale_factor = cfg.render_scale as f32 / 100.0;
    let width = (screen_width as f32 * scale_factor) as u32;
    let height = (screen_height as f32 * scale_factor) as u32;

    cmd.push_str(&format!("cd \"{gamedir}\"; "));
    for (i, p) in players.iter().enumerate() {
        let (gsc_width, gsc_height) = get_instance_resolution(players.len(), i, width, height);

        if gsc_height < 600 && res_warn {
            msg(
                "Resolution warning",
                "Instance resolution is below 600p! The game may experience graphical issues or not run at all. Increase the resolution scale in settings if this happens.",
            );
            res_warn = false;
        }

        cmd.push_str(&format!(
            "gamescope -W {gsc_width} -H {gsc_height} --backend=sdl -- "
        ));
        cmd.push_str(&format!(
            "bwrap --die-with-parent --dev-bind / / --tmpfs /tmp "
        ));

        // Bind player profile directories to the game's directories
        let mut binds = String::new();

        // Mask out any gamepads that aren't this player's
        for (i, pad) in all_pads.iter().enumerate() {
            if p.pad_index == i {
                continue;
            } else {
                let path = pad.path();
                binds.push_str(&format!("--bind /dev/null {path} "))
            }
        }

        cmd.push_str(&format!("{binds} {runtime} \"{exec}\""));

        if i < players.len() - 1 {
            // Proton games need a ~5 second buffer in-between launches
            // TODO: investigate why this is
            if win {
                cmd.push_str("& sleep 6; ");
            } else {
                cmd.push_str("& sleep 0.01; ");
            }
        }
    }

    Ok(cmd)
}
