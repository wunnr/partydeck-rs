use crate::paths::*;

use std::error::Error;

pub fn check_for_partydeck_update() -> bool {
    // Try to get the latest release tag from GitHub
    if let Ok(client) = reqwest::blocking::Client::new()
        .get("https://api.github.com/repos/wunnr/partydeck-rs/releases/latest")
        .header("User-Agent", "partydeck")
        .send()
    {
        if let Ok(release) = client.json::<serde_json::Value>() {
            // Extract the tag name (vX.X.X format)
            if let Some(tag_name) = release["tag_name"].as_str() {
                // Strip the 'v' prefix
                let latest_version = tag_name.strip_prefix('v').unwrap_or(tag_name);

                // Get current version from env!
                let current_version = env!("CARGO_PKG_VERSION");

                // Compare versions using semver
                if let (Ok(latest_semver), Ok(current_semver)) = (
                    semver::Version::parse(latest_version),
                    semver::Version::parse(current_version),
                ) {
                    return latest_semver > current_semver;
                }
            }
        }
    }

    // Default to false if any part of the process fails
    false
}

pub fn update_umu_launcher() -> Result<(), Box<dyn Error>> {
    use compress_tools::*;

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

    let umu_tar = std::fs::File::open(&tar_path)?;
    let dest_path = PATH_RES.join("umu-run");

    let mut dest_file = std::fs::File::create(&dest_path)?;
    uncompress_archive_file(&umu_tar, &mut dest_file, "umu/umu-run")?;

    // Set executable permissions
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&dest_path)?.permissions();
    perms.set_mode(0o755); // rwxr-xr-x permissions
    std::fs::set_permissions(&dest_path, perms)?;

    // Cleanup
    std::fs::remove_dir_all(tmp_dir)?;

    Ok(())
}

pub fn update_goldberg_emu() -> Result<(), Box<dyn Error>> {
    use compress_tools::*;

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

    let linux_release_tar = std::fs::File::open(&linux_path)?;
    uncompress_archive(linux_release_tar, &linux_dest, Ownership::Preserve)?;

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

    let win_release_7z = std::fs::File::open(&win_path)?;
    uncompress_archive(win_release_7z, &win_dest, Ownership::Preserve)?;

    // Cleanup
    std::fs::remove_dir_all(tmp_dir)?;

    Ok(())
}
