pub struct Player {
    pub pad_index: usize,
    pub profname: String,
    pub profselection: usize,
}

pub fn is_pad_in_players(index: usize, players: &Vec<Player>) -> bool {
    for player in players {
        if player.pad_index == index {
            return true;
        }
    }
    false
}

use evdev::*;

pub struct Gamepad {
    path: String,
    dev: Device,
}
pub enum PadButton {
    Left,
    Right,
    Up,
    Down,
    ABtn,
    BBtn,
    XBtn,
    YBtn,
    StartBtn,
    SelectBtn,
}
impl Gamepad {
    pub fn name(&self) -> &str {
        self.dev.name().unwrap_or_else(|| "")
    }
    pub fn fancyname(&self) -> &str {
        match self.dev.input_id().vendor() {
            0x045e => "Xbox Controller",
            0x054c => "PS Controller",
            0x057e => "NT Pro Controller",
            _ => self.name(),
        }
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn poll(&mut self) -> Option<PadButton> {
        let mut btn: Option<PadButton> = None;
        if let Ok(events) = self.dev.fetch_events() {
            for event in events {
                btn = match event.destructure() {
                    EventSummary::Key(_, KeyCode::BTN_SOUTH, 1) => Some(PadButton::ABtn),
                    EventSummary::Key(_, KeyCode::BTN_EAST, 1) => Some(PadButton::BBtn),
                    EventSummary::Key(_, KeyCode::BTN_NORTH, 1) => Some(PadButton::XBtn),
                    EventSummary::Key(_, KeyCode::BTN_WEST, 1) => Some(PadButton::YBtn),
                    EventSummary::Key(_, KeyCode::BTN_START, 1) => Some(PadButton::StartBtn),
                    EventSummary::Key(_, KeyCode::BTN_SELECT, 1) => Some(PadButton::SelectBtn),
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0X, -1) => {
                        Some(PadButton::Left)
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0X, 1) => {
                        Some(PadButton::Right)
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0Y, -1) => {
                        Some(PadButton::Up)
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_HAT0Y, 1) => {
                        Some(PadButton::Down)
                    }
                    _ => btn,
                };
            }
        }
        btn
    }
}

pub fn scan_evdev_gamepads() -> Vec<Gamepad> {
    let mut pads: Vec<Gamepad> = Vec::new();
    for dev in evdev::enumerate() {
        let has_btn_south = dev
            .1
            .supported_keys()
            .map_or(false, |keys| keys.contains(KeyCode::BTN_SOUTH));
        if has_btn_south {
            if dev.1.set_nonblocking(true).is_err() {
                println!("Failed to set non-blocking mode for {}", dev.0.display());
                continue;
            }
            pads.push(Gamepad {
                path: dev.0.to_str().unwrap().to_string(),
                dev: dev.1,
            });
        }
    }
    pads
}

#[allow(dead_code)]
pub fn scan_evdev_mice() -> Vec<Device> {
    let mut mice: Vec<Device> = Vec::new();
    for dev in evdev::enumerate() {
        let has_btn_left = dev
            .1
            .supported_keys()
            .map_or(false, |keys| keys.contains(KeyCode::BTN_LEFT));
        if has_btn_left {
            if dev.1.set_nonblocking(true).is_err() {
                println!("Failed to set non-blocking mode for {}", dev.0.display());
                continue;
            }
            mice.push(dev.1);
        }
    }
    mice
}
