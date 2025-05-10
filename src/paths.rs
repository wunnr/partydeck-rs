use crate::util::get_rootpath;
use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

// TODO: read resources from some place other than execdir/res?
pub static PATH_RES: LazyLock<PathBuf> =
    LazyLock::new(|| env::current_exe().unwrap().parent().unwrap().join("res"));
pub static PATH_HOME: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env::var("HOME").unwrap()));
pub static PATH_LOCAL_SHARE: LazyLock<PathBuf> = LazyLock::new(|| PATH_HOME.join(".local/share"));
pub static PATH_PARTY: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        return PathBuf::from(xdg_data_home).join("partydeck");
    }
    PATH_LOCAL_SHARE.join("partydeck")
});
pub static PATH_STEAM: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Ok(steam_path) = env::var("STEAM_BASE_FOLDER") {
        return PathBuf::from(steam_path);
    } else if PATH_LOCAL_SHARE.join("Steam").exists() {
        PATH_LOCAL_SHARE.join("Steam")
    } else if PATH_HOME
        .join(".var/app/com.valvesoftware.Steam/.steam/steam")
        .exists()
    {
        PATH_HOME.join(".var/app/com.valvesoftware.Steam/.steam/steam")
    } else {
        PathBuf::from(get_rootpath("steam").unwrap())
    }
});
