//! What the app calls things, in the user's language. The view models name nothing a user reads.

use presentation::{
    ActionCaption, ChannelPath, ControlTitle, EditorTab, InfoChannel, InfoCommands, InterfaceRole, KeyGroupKind,
    KeyUsage, KnobAction, LedColor, LogEvent, MediaChoice, ModifierKey, MouseChoice, Notice, SimulatorKind,
};

use crate::i18n::{t, tf};

pub fn control_title(title: ControlTitle) -> String {
    match title {
        ControlTitle::Key(number) => tf("Key %lld", &[&number]),
        ControlTitle::Knob(number, KnobAction::RotateLeft) => tf("Knob %lld · Rotate left", &[&number]),
        ControlTitle::Knob(number, KnobAction::Press) => tf("Knob %lld · Press", &[&number]),
        ControlTitle::Knob(number, KnobAction::RotateRight) => tf("Knob %lld · Rotate right", &[&number]),
        ControlTitle::Knob(number, KnobAction::PressRotateLeft) => tf("Knob %lld · Press and rotate left", &[&number]),
        ControlTitle::Knob(number, KnobAction::PressRotateRight) => {
            tf("Knob %lld · Press and rotate right", &[&number])
        }
    }
}

pub fn media_title(media: MediaChoice) -> String {
    match media {
        MediaChoice::PlayPause => t("Play/Pause"),
        MediaChoice::Stop => t("Stop"),
        MediaChoice::PreviousTrack => t("Prev Song"),
        MediaChoice::NextTrack => t("Next Song"),
        MediaChoice::Mute => t("Mute"),
        MediaChoice::VolumeUp => t("Volume+"),
        MediaChoice::VolumeDown => t("Volume-"),
        MediaChoice::BrightnessUp => t("Brightness+"),
        MediaChoice::BrightnessDown => t("Brightness-"),
        MediaChoice::BassUp => t("Bass+"),
        MediaChoice::BassDown => t("Bass-"),
        MediaChoice::TrebleUp => t("Treble+"),
        MediaChoice::TrebleDown => t("Treble-"),
        MediaChoice::MediaPlayer => t("Media Player"),
        MediaChoice::Email => t("Email"),
        MediaChoice::Calculator => t("Calculator"),
        MediaChoice::MyComputer => t("My Computer"),
        MediaChoice::WebHome => t("Web Home"),
        MediaChoice::WebBack => t("Web Back"),
        MediaChoice::WebForward => t("Web Forward"),
        MediaChoice::WebRefresh => t("Web Refresh"),
    }
}

/// An icon of the Adwaita theme.
pub fn media_icon(media: MediaChoice) -> &'static str {
    match media {
        MediaChoice::PlayPause => "media-playback-start-symbolic",
        MediaChoice::Stop => "media-playback-stop-symbolic",
        MediaChoice::PreviousTrack => "media-skip-backward-symbolic",
        MediaChoice::NextTrack => "media-skip-forward-symbolic",
        MediaChoice::Mute => "audio-volume-muted-symbolic",
        MediaChoice::VolumeUp => "audio-volume-high-symbolic",
        MediaChoice::VolumeDown => "audio-volume-low-symbolic",
        MediaChoice::BrightnessUp | MediaChoice::BrightnessDown => "display-brightness-symbolic",
        MediaChoice::BassUp | MediaChoice::BassDown | MediaChoice::TrebleUp | MediaChoice::TrebleDown => {
            "audio-speakers-symbolic"
        }
        MediaChoice::MediaPlayer => "applications-multimedia-symbolic",
        MediaChoice::Email => "mail-unread-symbolic",
        MediaChoice::Calculator => "accessories-calculator-symbolic",
        MediaChoice::MyComputer => "computer-symbolic",
        MediaChoice::WebHome => "go-home-symbolic",
        MediaChoice::WebBack => "go-previous-symbolic",
        MediaChoice::WebForward => "go-next-symbolic",
        MediaChoice::WebRefresh => "view-refresh-symbolic",
    }
}

pub fn mouse_title(mouse: MouseChoice) -> String {
    match mouse {
        MouseChoice::LeftClick => t("Mouse Left"),
        MouseChoice::MiddleClick => t("Mouse Centre"),
        MouseChoice::RightClick => t("Mouse Right"),
        MouseChoice::WheelUp => t("Mouse wheel Up"),
        MouseChoice::WheelDown => t("Mouse wheel Down"),
        MouseChoice::ControlWheelUp => t("Ctrl + Mouse wheel Up"),
        MouseChoice::ControlWheelDown => t("Ctrl + Mouse wheel Down"),
        MouseChoice::ShiftWheelUp => t("Shift + Mouse wheel Up"),
        MouseChoice::ShiftWheelDown => t("Shift + Mouse wheel Down"),
        MouseChoice::AltWheelUp => t("Alt + Mouse wheel Up"),
        MouseChoice::AltWheelDown => t("Alt + Mouse wheel Down"),
    }
}

pub fn mouse_icon(mouse: MouseChoice) -> &'static str {
    match mouse {
        MouseChoice::LeftClick | MouseChoice::MiddleClick | MouseChoice::RightClick => "input-mouse-symbolic",
        MouseChoice::WheelUp | MouseChoice::ControlWheelUp | MouseChoice::ShiftWheelUp | MouseChoice::AltWheelUp => {
            "go-up-symbolic"
        }
        MouseChoice::WheelDown
        | MouseChoice::ControlWheelDown
        | MouseChoice::ShiftWheelDown
        | MouseChoice::AltWheelDown => "go-down-symbolic",
    }
}

pub fn modifier_title(modifier: ModifierKey) -> String {
    match modifier {
        ModifierKey::Control => t("Control"),
        ModifierKey::Alt => t("Alt"),
        ModifierKey::Shift => t("Shift"),
        ModifierKey::Super => t("Super"),
        ModifierKey::RightControl => t("Right Control"),
        ModifierKey::RightAlt => t("Right Alt"),
        ModifierKey::RightShift => t("Right Shift"),
        ModifierKey::RightSuper => t("Right Super"),
    }
}

pub fn led_color_title(color: LedColor) -> String {
    match color {
        LedColor::Red => t("Red"),
        LedColor::Orange => t("Orange"),
        LedColor::Yellow => t("Yellow"),
        LedColor::Green => t("Green"),
        LedColor::Cyan => t("Cyan"),
        LedColor::Blue => t("Blue"),
        LedColor::Purple => t("Purple"),
    }
}

/// The colour a swatch shows, as CSS.
pub fn led_color_css(color: LedColor) -> &'static str {
    match color {
        LedColor::Red => "#e01b24",
        LedColor::Orange => "#ff7800",
        LedColor::Yellow => "#f6d32d",
        LedColor::Green => "#33d17a",
        LedColor::Cyan => "#33c7de",
        LedColor::Blue => "#3584e4",
        LedColor::Purple => "#9141ac",
    }
}

pub fn interface_role_title(role: InterfaceRole) -> String {
    match role {
        InterfaceRole::Keyboard => t("Keyboard"),
        InterfaceRole::Mouse => t("Mouse"),
        InterfaceRole::Media => t("Multimedia"),
        InterfaceRole::Configuration => t("Configuration Channel"),
        InterfaceRole::Other => t("Other"),
    }
}

/// The 1189:8890's light modes, named for what they do on one with six keys and a knob. Other keyboards'
/// modes have numbers only — what they do hasn't been seen.
pub fn lighting_mode_title(mode: u8, k8890: bool) -> String {
    match (k8890, mode) {
        (true, 0) => t("Off"),
        (true, 1) => t("Blink Last Key"),
        (true, 2) => t("Running Light"),
        _ => tf("Mode %lld", &[&mode]),
    }
}

pub fn lighting_mode_detail(mode: u8) -> Option<String> {
    match mode {
        1 => Some(t("The last key pressed keeps blinking, in a new color with each press.")),
        2 => Some(t("A light runs across the keys in order, in a new color each round.")),
        _ => None,
    }
}

pub fn lighting_mode_icon(mode: u8) -> &'static str {
    match mode {
        0 => "weather-clear-night-symbolic",
        1 => "input-keyboard-symbolic",
        _ => "media-playlist-repeat-symbolic",
    }
}

pub fn key_group_title(kind: KeyGroupKind) -> String {
    match kind {
        KeyGroupKind::Letters => t("Letters"),
        KeyGroupKind::Numbers => t("Numbers"),
        KeyGroupKind::Editing => t("Editing"),
        KeyGroupKind::Symbols => t("Symbols"),
        KeyGroupKind::FunctionKeys => t("Function Keys"),
        KeyGroupKind::Navigation => t("Navigation"),
        KeyGroupKind::Numpad => t("Numpad"),
    }
}

pub fn tab_title(tab: EditorTab) -> String {
    match tab {
        EditorTab::Keys => t("Keys"),
        EditorTab::Media => t("Multimedia"),
        EditorTab::Mouse => t("Mouse"),
        EditorTab::Lighting => t("LED"),
        EditorTab::Delay => t("Delay"),
    }
}

pub fn notice_message(notice: Notice) -> String {
    match notice {
        Notice::Saved { simulated: true } => t("Saved to the simulator — nothing was written to a keyboard."),
        Notice::Saved { simulated: false } => t("Saved to the keyboard."),
        Notice::ReadComplete => t("Read from the keyboard."),
        Notice::NothingToSave => t("Nothing to save — no key has changed."),
        Notice::IdentifyFailed => t("The keyboard couldn’t be asked what it is."),
        Notice::ReadFailed => t("Reading from the keyboard failed."),
        Notice::WriteFailed(code) => tf("Writing to the keyboard failed (%@).", &[&error_code(code)]),
        Notice::KeyboardGone => t("The keyboard is no longer connected."),
        Notice::Full(limit) => tf("A key holds up to %lld presses.", &[&limit]),
        Notice::SelectControlFirst => t("Select a key or knob."),
        Notice::ModifiersOnlyAlone => {
            t("Modifiers alone make the key a modifier key, so they can’t go with other presses.")
        }
        Notice::NeedsColorKey(keys) => tf("Only keys 1–%lld have a color. Select one of them.", &[&keys]),
        Notice::Unsupported => t("This keyboard can’t do that."),
    }
}

/// What a key cap shows: "Super+C", "Mute".
pub fn caption_text(caption: &ActionCaption) -> String {
    match caption {
        ActionCaption::Unknown | ActionCaption::Nothing => String::new(),
        ActionCaption::Presses(text) => text.clone(),
        ActionCaption::Media(media) => media_title(*media),
        ActionCaption::OtherMedia(usage) => tf("Media Key %@", &[&format!("0x{usage:03X}")]),
        ActionCaption::Mouse(mouse) => mouse_title(*mouse),
    }
}

/// What the key is called, for its tooltip. A key whose cap reads plainly — a letter, a digit, F5 — goes by
/// its label.
pub fn key_name(key: &KeyUsage) -> String {
    match key.usage {
        0x28 => t("Enter"),
        0x29 => t("Escape"),
        0x2A => t("Backspace"),
        0x2B => t("Tab"),
        0x2C => t("Space"),
        0x39 => t("Caps Lock"),
        0x65 => t("Menu Key"),
        0x46 => t("Print Screen"),
        0x47 => t("Scroll Lock"),
        0x49 => t("Insert"),
        0x4A => t("Home"),
        0x4B => t("Page Up"),
        0x4C => t("Delete"),
        0x4D => t("End"),
        0x4E => t("Page Down"),
        0x4F => t("Right Arrow"),
        0x50 => t("Left Arrow"),
        0x51 => t("Down Arrow"),
        0x52 => t("Up Arrow"),
        0x53 => t("Num Lock"),
        0x58 => t("Keypad Enter"),
        0x54..=0x63 => {
            // The keypad's operators as their keys are usually named, rather than as their caps show them.
            let symbol = match key.usage {
                0x54 => "/",
                0x55 => "*",
                0x56 => "-",
                0x57 => "+",
                _ => key.label,
            };
            tf("Keypad %@", &[&symbol])
        }
        0x32 | 0x64 => tf("Non-US Key %@ %@", &[&key.label, &key.shifted.unwrap_or_default()]),
        _ => key.label.to_owned(),
    }
}

pub fn log_event_message(event: &LogEvent) -> String {
    match event {
        LogEvent::Searching => t("Looking for the keyboard"),
        LogEvent::Connected { usb, interface } => tf("Connected to %@, interface %lld", &[usb, interface]),
        LogEvent::NeedsPermission => t("Access to the keyboard is needed"),
        LogEvent::OpenFailed { code } => tf("The keyboard couldn’t be opened (%@).", &[&error_code(*code)]),
        LogEvent::WriteFailed { code } => tf("Writing to the keyboard failed (%@).", &[&error_code(*code)]),
        LogEvent::Simulated { id } => tf("Simulated keyboard: %@", &[id]),
    }
}

/// A failure's code: an errno ("errno 13: Permission denied"), or libusb's (negative).
pub fn error_code(code: i32) -> String {
    if code < 0 {
        format!("libusb {code}")
    } else {
        format!("errno {code}: {}", std::io::Error::from_raw_os_error(code))
    }
}

pub fn simulator_kind_title(kind: &SimulatorKind) -> String {
    match kind {
        SimulatorKind::K8890 { report_id } => tf("Report ID %lld", &[report_id]),
        SimulatorKind::Identified { keys, knobs, kind, usb } => {
            tf("%@ · %@", &[&tf("Keys: %lld · Knobs: %lld · Type %lld", &[keys, knobs, kind]), usb])
        }
        SimulatorKind::Unidentified { usb } => tf("%@ · %@", &[&commands_text(InfoCommands::Unidentified), usb]),
    }
}

/// "Interface 1 · OUT 0x02 · Report ID 3 · 65 bytes".
pub fn channel_text(channel: &InfoChannel) -> String {
    let way = match channel.path {
        ChannelPath::Usb { out_endpoint } => format!("OUT 0x{out_endpoint:02X}"),
        ChannelPath::Hid => "hidraw".to_owned(),
        ChannelPath::Simulated => t("Simulator"),
    };
    tf(
        "Interface %lld · %@ · %@ · %@",
        &[
            &channel.interface,
            &way,
            &tf("Report ID %lld", &[&channel.report_id]),
            &tf("%lld bytes", &[&channel.report_length]),
        ],
    )
}

pub fn commands_text(commands: InfoCommands) -> String {
    match commands {
        InfoCommands::Packets { report_id } => tf("8890 Packets · %@", &[&tf("Report ID %lld", &[&report_id])]),
        InfoCommands::Standard => t("884x · Standard Firmware (Type 10)"),
        InfoCommands::Legacy => t("884x · Legacy Firmware"),
        InfoCommands::Unidentified => t("884x · Doesn’t Answer Identification"),
        InfoCommands::Unknown => "—".to_owned(),
    }
}
