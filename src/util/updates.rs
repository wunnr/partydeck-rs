use std::error::Error;
use std::io::ErrorKind;
use crate::paths::*;

pub fn update_umu_launcher() -> Result<(), Box<dyn Error>> {
    print!("Updating UMU Launcher...");
    if !PATH_RES.exists() {
        std::fs::create_dir_all(PATH_RES.clone())?;
    }
    // Get latest release info from GitHub API
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/Open-Wine-Components/umu-launcher/releases/latest")
        .header("User-Agent", "partydeck")
        .send()?;

    let release: serde_json::Value = response.json()?;

    // Find zipapp asset URL
    let assets = release["assets"].as_array().ok_or("No assets found")?;
    let zipapp_url = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map(|name| name.contains("zipapp.tar"))
                .unwrap_or(false)
        })
        .and_then(|asset| asset["browser_download_url"].as_str())
        .ok_or("Zipapp not found in release")?;

    // Download the file
    let tmp_dir = PATH_PARTY.join("tmp");
    if !tmp_dir.exists() {
        std::fs::create_dir_all(&tmp_dir)?;
    }

    let tar_path = tmp_dir.join("umu-launcher.tar");
    let mut response = client.get(zipapp_url).send()?;
    let mut file = std::fs::File::create(&tar_path)?;
    std::io::copy(&mut response, &mut file)?;

    // Extract tar file
    let tar_file = std::fs::File::open(&tar_path)?;
    let mut archive = tar::Archive::new(tar_file);
    archive.unpack(&tmp_dir)?;

    // Copy umu-run to data directory
    std::fs::copy(tmp_dir.join("umu/umu-run"), PATH_RES.join("umu-run"))?;

    // Cleanup
    std::fs::remove_dir_all(tmp_dir)?;

    Ok(())
}

pub fn update_goldberg_emu() -> Result<(), Box<dyn Error>> {
    print!("Updating Goldberg Emulator...");
    // Get latest release info from GitHub API
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/Detanup01/gbe_fork/releases/latest")
        .header("User-Agent", "partydeck")
        .send()?;

    let release: serde_json::Value = response.json()?;

    // Find asset URLs
    let assets = release["assets"].as_array().ok_or("No assets found")?;

    let linux_url = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map(|name| name == "emu-linux-release.tar.bz2")
                .unwrap_or(false)
        })
        .and_then(|asset| asset["browser_download_url"].as_str())
        .ok_or("Linux release not found")?;

    let win_url = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map(|name| name == "emu-win-release.7z")
                .unwrap_or(false)
        })
        .and_then(|asset| asset["browser_download_url"].as_str())
        .ok_or("Windows release not found")?;

    // Create temp directory
    let tmp_dir = PATH_PARTY.join("tmp");
    if !tmp_dir.exists() {
        std::fs::create_dir_all(&tmp_dir)?;
    }

    // Download and extract Linux release
    let linux_path = tmp_dir.join("emu-linux-release.tar.bz2");
    let mut response = client.get(linux_url).send()?;
    let mut file = std::fs::File::create(&linux_path)?;
    std::io::copy(&mut response, &mut file)?;

    let linux_dest = PATH_RES.join("goldberg_linux");
    if linux_dest.exists() {
        std::fs::remove_dir_all(&linux_dest)?;
    }
    std::fs::create_dir_all(&linux_dest)?;

    // First decompress the bz2
    let status = std::process::Command::new("7z")
        .arg("x")
        .arg(&linux_path)
        .arg(format!("-o{}", tmp_dir.display()))
        .spawn();

    if let Err(error) = status {
        return match error.kind() {
            ErrorKind::NotFound => Err("7z executable could not be found. Install 7z and try again".into()),
            _ => Err("Failed to decompress Linux release".into())
        }
    }

    // Then extract the tar
    let tar_path = tmp_dir.join("emu-linux-release.tar");
    let status = std::process::Command::new("tar")
        .arg("xf")
        .arg(&tar_path)
        .arg("-C")
        .arg(&linux_dest)
        .status()?;

    if !status.success() {
        return Err("Failed to extract Linux release".into());
    }

    // Download and extract Windows release
    let win_path = tmp_dir.join("emu-win-release.7z");
    let mut response = client.get(win_url).send()?;
    let mut file = std::fs::File::create(&win_path)?;
    std::io::copy(&mut response, &mut file)?;

    let win_dest = PATH_RES.join("goldberg_win");
    if win_dest.exists() {
        std::fs::remove_dir_all(&win_dest)?;
    }
    std::fs::create_dir_all(&win_dest)?;

    let status = std::process::Command::new("7z")
        .arg("x")
        .arg(&win_path)
        .arg(format!("-o{}", win_dest.display()))
        .status()?;

    if !status.success() {
        return Err("Failed to extract Windows release".into());
    }

    // Cleanup
    std::fs::remove_dir_all(tmp_dir)?;

    Ok(())
}
