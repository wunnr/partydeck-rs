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

// Self updater for portable version will eventually go here
