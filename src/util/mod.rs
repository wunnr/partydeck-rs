// Re-export all utility functions from submodules
mod filesystem;
mod profiles;
mod sys;
mod updates;

// Re-export functions from profiles
pub use profiles::{create_gamesave, create_profile, remove_guest_profiles, scan_profiles};

// Re-export functions from filesystem
pub use filesystem::{SanitizePath, copy_dir_recursive, get_rootpath, get_rootpath_handler};

// Re-export functions from launcher
pub use sys::{
    create_proton_pfx, get_instance_resolution, get_screen_resolution, kwin_dbus_start_script,
    kwin_dbus_unload_script, msg, yesno,
};

// Re-export functions from updates
pub use updates::{check_for_partydeck_update, update_goldberg_emu, update_umu_launcher};
