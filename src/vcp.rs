use serde::{Deserialize, Serialize};

pub const KNOWN_FEATURES: [VcpInfo; 28] = [
    VcpInfo {
        code: 0x10,
        name: "Brightness",
        kind: VcpKind::Continuous,
        group: VcpGroup::Base,
    },
    VcpInfo {
        code: 0x12,
        name: "Contrast",
        kind: VcpKind::Continuous,
        group: VcpGroup::Base,
    },
    VcpInfo {
        code: 0x60,
        name: "Input Source",
        kind: VcpKind::Enum,
        group: VcpGroup::InputPower,
    },
    VcpInfo {
        code: 0xD6,
        name: "Power Mode",
        kind: VcpKind::Enum,
        group: VcpGroup::InputPower,
    },
    VcpInfo {
        code: 0x62,
        name: "Audio Speaker Volume",
        kind: VcpKind::Continuous,
        group: VcpGroup::InputPower,
    },
    VcpInfo {
        code: 0x14,
        name: "Select Color Preset",
        kind: VcpKind::Enum,
        group: VcpGroup::Color,
    },
    VcpInfo {
        code: 0x16,
        name: "Video Gain (Red)",
        kind: VcpKind::Continuous,
        group: VcpGroup::Color,
    },
    VcpInfo {
        code: 0x18,
        name: "Video Gain (Green)",
        kind: VcpKind::Continuous,
        group: VcpGroup::Color,
    },
    VcpInfo {
        code: 0x1A,
        name: "Video Gain (Blue)",
        kind: VcpKind::Continuous,
        group: VcpGroup::Color,
    },
    VcpInfo {
        code: 0x0B,
        name: "Color Temperature Increment",
        kind: VcpKind::Continuous,
        group: VcpGroup::Color,
    },
    VcpInfo {
        code: 0x0C,
        name: "Save Current Settings",
        kind: VcpKind::Action,
        group: VcpGroup::Reset,
    },
    VcpInfo {
        code: 0xCA,
        name: "OSD / Button Control",
        kind: VcpKind::Enum,
        group: VcpGroup::SystemOsd,
    },
    VcpInfo {
        code: 0xCC,
        name: "OSD Language",
        kind: VcpKind::Enum,
        group: VcpGroup::SystemOsd,
    },
    VcpInfo {
        code: 0x8D,
        name: "Audio Mute / Screen Blank",
        kind: VcpKind::Enum,
        group: VcpGroup::SystemOsd,
    },
    VcpInfo {
        code: 0x02,
        name: "New Control Value",
        kind: VcpKind::Action,
        group: VcpGroup::Reset,
    },
    VcpInfo {
        code: 0x04,
        name: "Restore Factory Defaults",
        kind: VcpKind::Action,
        group: VcpGroup::Reset,
    },
    VcpInfo {
        code: 0x05,
        name: "Restore Brightness/Contrast Defaults",
        kind: VcpKind::Action,
        group: VcpGroup::Reset,
    },
    VcpInfo {
        code: 0x08,
        name: "Restore Color Defaults",
        kind: VcpKind::Action,
        group: VcpGroup::Reset,
    },
    VcpInfo {
        code: 0xAC,
        name: "Horizontal Frequency",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xAE,
        name: "Vertical Frequency",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xB2,
        name: "Flat Panel Sub-Pixel Layout",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xB6,
        name: "Display Technology Type",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xC0,
        name: "Display Usage Time",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xC6,
        name: "Application Enable Key",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xC8,
        name: "Display Controller Type",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xC9,
        name: "Display Firmware Level",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0xDF,
        name: "VCP Version",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
    VcpInfo {
        code: 0x52,
        name: "Active Control",
        kind: VcpKind::ReadOnly,
        group: VcpGroup::Info,
    },
];

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum VcpKind {
    Continuous, // Single range value (es. slider)
    Enum,       // Discrete values set (es. ComboBox)
    ReadOnly,   // Shown by the display but not writeble.
    Action,     // "Command" that trigger an action (ex. restore defaults)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum VcpGroup {
    Base,
    InputPower,
    Color,
    SystemOsd,
    Reset,
    Info,
}

impl VcpGroup {
    pub fn label(&self) -> &'static str {
        match self {
            VcpGroup::Base => "Base",
            VcpGroup::InputPower => "Input / Power",
            VcpGroup::Color => "Color",
            VcpGroup::SystemOsd => "System / OSD",
            VcpGroup::Reset => "Reset",
            VcpGroup::Info => "Info",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            VcpGroup::Base => "☀",
            VcpGroup::InputPower => "⏻",
            VcpGroup::Color => "🎨",
            VcpGroup::SystemOsd => "⚙",
            VcpGroup::Reset => "↺",
            VcpGroup::Info => "ℹ",
        }
    }

    pub fn list() -> Vec<Self> {
        Vec::from([
            VcpGroup::Base,
            VcpGroup::InputPower,
            VcpGroup::Color,
            VcpGroup::SystemOsd,
            VcpGroup::Reset,
            VcpGroup::Info,
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VcpInfo {
    pub code: u8,
    pub name: &'static str,
    pub kind: VcpKind, // Continuous o Enum
    pub group: VcpGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VcpRuntime {
    pub current: u16,
    pub max: u16,
    pub allowed: Vec<u16>,
}

impl VcpInfo {
    pub fn by_group(group: VcpGroup) -> Vec<&'static VcpInfo> {
        KNOWN_FEATURES
            .iter()
            .filter(|info| info.group == group)
            .collect()
    }

    pub fn enum_label(code: u8, value: u16) -> Option<&'static str> {
        match (code, value) {
            (0x60, 1) => Some("VGA-1"),
            (0x60, 2) => Some("VGA-2"),
            (0x60, 3) => Some("DVI-1"),
            (0x60, 4) => Some("DVI-2"),
            (0x60, 5) => Some("Composite-1"),
            (0x60, 6) => Some("Composite-2"),
            (0x60, 7) => Some("S-Video-1"),
            (0x60, 8) => Some("S-Video-2"),
            (0x60, 9) => Some("Tuner-1"),
            (0x60, 10) => Some("Tuner-2"),
            (0x60, 11) => Some("Tuner-3"),
            (0x60, 12) => Some("Component-1"),
            (0x60, 13) => Some("Component-2"),
            (0x60, 14) => Some("Component-3"),
            (0x60, 15) => Some("DisplayPort-1"),
            (0x60, 16) => Some("DisplayPort-2"),
            (0x60, 17) => Some("HDMI-1"),
            (0x60, 18) => Some("HDMI-2"),

            (0x14, 1) => Some("sRGB"),
            (0x14, 2) => Some("Display Native"),
            (0x14, 3) => Some("4000 K"),
            (0x14, 4) => Some("5000 K"),
            (0x14, 5) => Some("6500 K"),
            (0x14, 6) => Some("7500 K"),
            (0x14, 7) => Some("8200 K"),
            (0x14, 8) => Some("9300 K"),
            (0x14, 9) => Some("10000 K"),
            (0x14, 10) => Some("11500 K"),
            (0x14, 11) => Some("User 1"),
            (0x14, 12) => Some("User 2"),
            (0x14, 13) => Some("User 3"),

            (0x8D, 1) => Some("Muted"),
            (0x8D, 2) => Some("Unmuted"),

            (0xCA, 1) => Some("Enabled"),
            (0xCA, 2) => Some("Disabled"),

            (0xD6, 1) => Some("On"),
            (0xD6, 2) => Some("Standby"),
            (0xD6, 3) => Some("Suspend"),
            (0xD6, 4) => Some("Off (soft)"),
            (0xD6, 5) => Some("Off (switch)"),

            (0xCC, 1) => Some("Chinese (Trad.)"),
            (0xCC, 2) => Some("English"),
            (0xCC, 3) => Some("French"),
            (0xCC, 4) => Some("German"),
            (0xCC, 5) => Some("Italian"),
            (0xCC, 6) => Some("Japanese"),
            (0xCC, 7) => Some("Korean"),
            (0xCC, 8) => Some("Portuguese"),
            (0xCC, 9) => Some("Russian"),
            (0xCC, 10) => Some("Chinese (Simpl.)"),
            (0xCC, 11) => Some("Spanish"),
            (0xCC, 12) => Some("Swedish"),
            (0xCC, 13) => Some("Turkish"),

            _ => None,
        }
    }
}
