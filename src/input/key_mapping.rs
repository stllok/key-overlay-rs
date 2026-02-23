//! Key identifier mapping for config strings and backend key codes.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyId {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Tab,
    Backspace,
    Escape,
    LShift,
    RShift,
    LControl,
    RControl,
    LAlt,
    RAlt,
    Mouse1,
    Mouse2,
    Mouse3,
    Mouse4,
    Mouse5,
}

impl FromStr for KeyId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_uppercase();

        match normalized.as_str() {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "H" => Ok(Self::H),
            "I" => Ok(Self::I),
            "J" => Ok(Self::J),
            "K" => Ok(Self::K),
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            "N" => Ok(Self::N),
            "O" => Ok(Self::O),
            "P" => Ok(Self::P),
            "Q" => Ok(Self::Q),
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "T" => Ok(Self::T),
            "U" => Ok(Self::U),
            "V" => Ok(Self::V),
            "W" => Ok(Self::W),
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            "0" | "D0" | "NUM0" => Ok(Self::D0),
            "1" | "D1" | "NUM1" => Ok(Self::D1),
            "2" | "D2" | "NUM2" => Ok(Self::D2),
            "3" | "D3" | "NUM3" => Ok(Self::D3),
            "4" | "D4" | "NUM4" => Ok(Self::D4),
            "5" | "D5" | "NUM5" => Ok(Self::D5),
            "6" | "D6" | "NUM6" => Ok(Self::D6),
            "7" | "D7" | "NUM7" => Ok(Self::D7),
            "8" | "D8" | "NUM8" => Ok(Self::D8),
            "9" | "D9" | "NUM9" => Ok(Self::D9),
            "F1" => Ok(Self::F1),
            "F2" => Ok(Self::F2),
            "F3" => Ok(Self::F3),
            "F4" => Ok(Self::F4),
            "F5" => Ok(Self::F5),
            "F6" => Ok(Self::F6),
            "F7" => Ok(Self::F7),
            "F8" => Ok(Self::F8),
            "F9" => Ok(Self::F9),
            "F10" => Ok(Self::F10),
            "F11" => Ok(Self::F11),
            "F12" => Ok(Self::F12),
            "SPACE" => Ok(Self::Space),
            "ENTER" | "RETURN" => Ok(Self::Enter),
            "TAB" => Ok(Self::Tab),
            "BACKSPACE" => Ok(Self::Backspace),
            "ESC" | "ESCAPE" => Ok(Self::Escape),
            "LSHIFT" => Ok(Self::LShift),
            "RSHIFT" => Ok(Self::RShift),
            "LCONTROL" | "LCTRL" => Ok(Self::LControl),
            "RCONTROL" | "RCTRL" => Ok(Self::RControl),
            "LALT" => Ok(Self::LAlt),
            "RALT" | "ALTGR" => Ok(Self::RAlt),
            "MOUSE1" => Ok(Self::Mouse1),
            "MOUSE2" => Ok(Self::Mouse2),
            "MOUSE3" => Ok(Self::Mouse3),
            "MOUSE4" => Ok(Self::Mouse4),
            "MOUSE5" => Ok(Self::Mouse5),
            _ => Err(format!(
                "unsupported key name '{s}' (examples: A, 0, F1, LControl, Mouse1)"
            )),
        }
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::D0 => "0",
            Self::D1 => "1",
            Self::D2 => "2",
            Self::D3 => "3",
            Self::D4 => "4",
            Self::D5 => "5",
            Self::D6 => "6",
            Self::D7 => "7",
            Self::D8 => "8",
            Self::D9 => "9",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::Space => "Space",
            Self::Enter => "Enter",
            Self::Tab => "Tab",
            Self::Backspace => "Backspace",
            Self::Escape => "Escape",
            Self::LShift => "LShift",
            Self::RShift => "RShift",
            Self::LControl => "LControl",
            Self::RControl => "RControl",
            Self::LAlt => "LAlt",
            Self::RAlt => "RAlt",
            Self::Mouse1 => "Mouse1",
            Self::Mouse2 => "Mouse2",
            Self::Mouse3 => "Mouse3",
            Self::Mouse4 => "Mouse4",
            Self::Mouse5 => "Mouse5",
        };

        f.write_str(label)
    }
}

impl TryFrom<rdev::Key> for KeyId {
    type Error = String;

    fn try_from(value: rdev::Key) -> Result<Self, Self::Error> {
        match value {
            rdev::Key::KeyA => Ok(Self::A),
            rdev::Key::KeyB => Ok(Self::B),
            rdev::Key::KeyC => Ok(Self::C),
            rdev::Key::KeyD => Ok(Self::D),
            rdev::Key::KeyE => Ok(Self::E),
            rdev::Key::KeyF => Ok(Self::F),
            rdev::Key::KeyG => Ok(Self::G),
            rdev::Key::KeyH => Ok(Self::H),
            rdev::Key::KeyI => Ok(Self::I),
            rdev::Key::KeyJ => Ok(Self::J),
            rdev::Key::KeyK => Ok(Self::K),
            rdev::Key::KeyL => Ok(Self::L),
            rdev::Key::KeyM => Ok(Self::M),
            rdev::Key::KeyN => Ok(Self::N),
            rdev::Key::KeyO => Ok(Self::O),
            rdev::Key::KeyP => Ok(Self::P),
            rdev::Key::KeyQ => Ok(Self::Q),
            rdev::Key::KeyR => Ok(Self::R),
            rdev::Key::KeyS => Ok(Self::S),
            rdev::Key::KeyT => Ok(Self::T),
            rdev::Key::KeyU => Ok(Self::U),
            rdev::Key::KeyV => Ok(Self::V),
            rdev::Key::KeyW => Ok(Self::W),
            rdev::Key::KeyX => Ok(Self::X),
            rdev::Key::KeyY => Ok(Self::Y),
            rdev::Key::KeyZ => Ok(Self::Z),
            rdev::Key::Num0 => Ok(Self::D0),
            rdev::Key::Num1 => Ok(Self::D1),
            rdev::Key::Num2 => Ok(Self::D2),
            rdev::Key::Num3 => Ok(Self::D3),
            rdev::Key::Num4 => Ok(Self::D4),
            rdev::Key::Num5 => Ok(Self::D5),
            rdev::Key::Num6 => Ok(Self::D6),
            rdev::Key::Num7 => Ok(Self::D7),
            rdev::Key::Num8 => Ok(Self::D8),
            rdev::Key::Num9 => Ok(Self::D9),
            rdev::Key::F1 => Ok(Self::F1),
            rdev::Key::F2 => Ok(Self::F2),
            rdev::Key::F3 => Ok(Self::F3),
            rdev::Key::F4 => Ok(Self::F4),
            rdev::Key::F5 => Ok(Self::F5),
            rdev::Key::F6 => Ok(Self::F6),
            rdev::Key::F7 => Ok(Self::F7),
            rdev::Key::F8 => Ok(Self::F8),
            rdev::Key::F9 => Ok(Self::F9),
            rdev::Key::F10 => Ok(Self::F10),
            rdev::Key::F11 => Ok(Self::F11),
            rdev::Key::F12 => Ok(Self::F12),
            rdev::Key::Space => Ok(Self::Space),
            rdev::Key::Return | rdev::Key::KpReturn => Ok(Self::Enter),
            rdev::Key::Tab => Ok(Self::Tab),
            rdev::Key::Backspace => Ok(Self::Backspace),
            rdev::Key::Escape => Ok(Self::Escape),
            rdev::Key::ShiftLeft => Ok(Self::LShift),
            rdev::Key::ShiftRight => Ok(Self::RShift),
            rdev::Key::ControlLeft => Ok(Self::LControl),
            rdev::Key::ControlRight => Ok(Self::RControl),
            rdev::Key::Alt => Ok(Self::LAlt),
            rdev::Key::AltGr => Ok(Self::RAlt),
            _ => Err(format!("unsupported rdev key: {value:?}")),
        }
    }
}

impl From<KeyId> for rdev::Key {
    fn from(value: KeyId) -> Self {
        match value {
            KeyId::A => rdev::Key::KeyA,
            KeyId::B => rdev::Key::KeyB,
            KeyId::C => rdev::Key::KeyC,
            KeyId::D => rdev::Key::KeyD,
            KeyId::E => rdev::Key::KeyE,
            KeyId::F => rdev::Key::KeyF,
            KeyId::G => rdev::Key::KeyG,
            KeyId::H => rdev::Key::KeyH,
            KeyId::I => rdev::Key::KeyI,
            KeyId::J => rdev::Key::KeyJ,
            KeyId::K => rdev::Key::KeyK,
            KeyId::L => rdev::Key::KeyL,
            KeyId::M => rdev::Key::KeyM,
            KeyId::N => rdev::Key::KeyN,
            KeyId::O => rdev::Key::KeyO,
            KeyId::P => rdev::Key::KeyP,
            KeyId::Q => rdev::Key::KeyQ,
            KeyId::R => rdev::Key::KeyR,
            KeyId::S => rdev::Key::KeyS,
            KeyId::T => rdev::Key::KeyT,
            KeyId::U => rdev::Key::KeyU,
            KeyId::V => rdev::Key::KeyV,
            KeyId::W => rdev::Key::KeyW,
            KeyId::X => rdev::Key::KeyX,
            KeyId::Y => rdev::Key::KeyY,
            KeyId::Z => rdev::Key::KeyZ,
            KeyId::D0 => rdev::Key::Num0,
            KeyId::D1 => rdev::Key::Num1,
            KeyId::D2 => rdev::Key::Num2,
            KeyId::D3 => rdev::Key::Num3,
            KeyId::D4 => rdev::Key::Num4,
            KeyId::D5 => rdev::Key::Num5,
            KeyId::D6 => rdev::Key::Num6,
            KeyId::D7 => rdev::Key::Num7,
            KeyId::D8 => rdev::Key::Num8,
            KeyId::D9 => rdev::Key::Num9,
            KeyId::F1 => rdev::Key::F1,
            KeyId::F2 => rdev::Key::F2,
            KeyId::F3 => rdev::Key::F3,
            KeyId::F4 => rdev::Key::F4,
            KeyId::F5 => rdev::Key::F5,
            KeyId::F6 => rdev::Key::F6,
            KeyId::F7 => rdev::Key::F7,
            KeyId::F8 => rdev::Key::F8,
            KeyId::F9 => rdev::Key::F9,
            KeyId::F10 => rdev::Key::F10,
            KeyId::F11 => rdev::Key::F11,
            KeyId::F12 => rdev::Key::F12,
            KeyId::Space => rdev::Key::Space,
            KeyId::Enter => rdev::Key::Return,
            KeyId::Tab => rdev::Key::Tab,
            KeyId::Backspace => rdev::Key::Backspace,
            KeyId::Escape => rdev::Key::Escape,
            KeyId::LShift => rdev::Key::ShiftLeft,
            KeyId::RShift => rdev::Key::ShiftRight,
            KeyId::LControl => rdev::Key::ControlLeft,
            KeyId::RControl => rdev::Key::ControlRight,
            KeyId::LAlt => rdev::Key::Alt,
            KeyId::RAlt => rdev::Key::AltGr,
            KeyId::Mouse1 => rdev::Key::Unknown(0xF001),
            KeyId::Mouse2 => rdev::Key::Unknown(0xF002),
            KeyId::Mouse3 => rdev::Key::Unknown(0xF003),
            KeyId::Mouse4 => rdev::Key::Unknown(0xF004),
            KeyId::Mouse5 => rdev::Key::Unknown(0xF005),
        }
    }
}

impl TryFrom<rdev::Button> for KeyId {
    type Error = String;

    fn try_from(value: rdev::Button) -> Result<Self, Self::Error> {
        match value {
            rdev::Button::Left => Ok(Self::Mouse1),
            rdev::Button::Right => Ok(Self::Mouse2),
            rdev::Button::Middle => Ok(Self::Mouse3),
            rdev::Button::Unknown(4) => Ok(Self::Mouse4),
            rdev::Button::Unknown(5) => Ok(Self::Mouse5),
            _ => Err(format!("unsupported rdev mouse button: {value:?}")),
        }
    }
}

impl TryFrom<KeyId> for rdev::Button {
    type Error = String;

    fn try_from(value: KeyId) -> Result<Self, Self::Error> {
        match value {
            KeyId::Mouse1 => Ok(rdev::Button::Left),
            KeyId::Mouse2 => Ok(rdev::Button::Right),
            KeyId::Mouse3 => Ok(rdev::Button::Middle),
            KeyId::Mouse4 => Ok(rdev::Button::Unknown(4)),
            KeyId::Mouse5 => Ok(rdev::Button::Unknown(5)),
            _ => Err(format!("key is not a mouse button: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeyId;

    use rdev::{Button, Key};
    use std::str::FromStr;

    #[test]
    fn test_key_mapping_parse_original_names() {
        let cases = [
            ("A", KeyId::A),
            ("Z", KeyId::Z),
            ("0", KeyId::D0),
            ("9", KeyId::D9),
            ("D1", KeyId::D1),
            ("F1", KeyId::F1),
            ("F12", KeyId::F12),
            ("Space", KeyId::Space),
            ("Enter", KeyId::Enter),
            ("Tab", KeyId::Tab),
            ("Backspace", KeyId::Backspace),
            ("Escape", KeyId::Escape),
            ("LShift", KeyId::LShift),
            ("RShift", KeyId::RShift),
            ("LControl", KeyId::LControl),
            ("RControl", KeyId::RControl),
            ("LAlt", KeyId::LAlt),
            ("RAlt", KeyId::RAlt),
            ("Mouse1", KeyId::Mouse1),
            ("Mouse2", KeyId::Mouse2),
            ("Mouse3", KeyId::Mouse3),
            ("Mouse4", KeyId::Mouse4),
            ("Mouse5", KeyId::Mouse5),
        ];

        for (input, expected) in cases {
            let parsed = KeyId::from_str(input).unwrap_or_else(|err| {
                panic!("expected key {input} to parse, got error: {err}");
            });
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_key_mapping_parse_is_case_insensitive_and_trimmed() {
        assert_eq!(KeyId::from_str("  lcontrol  "), Ok(KeyId::LControl));
        assert_eq!(KeyId::from_str("mouse5"), Ok(KeyId::Mouse5));
        assert_eq!(KeyId::from_str("f10"), Ok(KeyId::F10));
    }

    #[test]
    fn test_key_mapping_unknown_name_returns_descriptive_error() {
        let error = KeyId::from_str("NotARealKey").expect_err("expected invalid key to fail");

        assert!(error.contains("NotARealKey"));
        assert!(error.contains("unsupported key name"));
    }

    #[test]
    fn test_key_mapping_display_uses_original_overlay_labels() {
        assert_eq!(KeyId::A.to_string(), "A");
        assert_eq!(KeyId::D4.to_string(), "4");
        assert_eq!(KeyId::F12.to_string(), "F12");
        assert_eq!(KeyId::LControl.to_string(), "LControl");
        assert_eq!(KeyId::Mouse3.to_string(), "Mouse3");
    }

    #[test]
    fn test_key_mapping_rdev_try_from_for_supported_keyboard_keys() {
        let cases = [
            (Key::KeyA, KeyId::A),
            (Key::KeyZ, KeyId::Z),
            (Key::Num0, KeyId::D0),
            (Key::Num9, KeyId::D9),
            (Key::F1, KeyId::F1),
            (Key::F12, KeyId::F12),
            (Key::Space, KeyId::Space),
            (Key::Return, KeyId::Enter),
            (Key::Tab, KeyId::Tab),
            (Key::Backspace, KeyId::Backspace),
            (Key::Escape, KeyId::Escape),
            (Key::ShiftLeft, KeyId::LShift),
            (Key::ShiftRight, KeyId::RShift),
            (Key::ControlLeft, KeyId::LControl),
            (Key::ControlRight, KeyId::RControl),
            (Key::Alt, KeyId::LAlt),
            (Key::AltGr, KeyId::RAlt),
        ];

        for (input, expected) in cases {
            assert_eq!(KeyId::try_from(input), Ok(expected));
        }
    }

    #[test]
    fn test_key_mapping_rdev_try_from_unknown_key_returns_error() {
        let error =
            KeyId::try_from(Key::Unknown(999)).expect_err("expected unknown key to return error");

        assert!(error.contains("unsupported rdev key"));
    }

    #[test]
    fn test_key_mapping_into_rdev_round_trips_keyboard_keys() {
        let keys = [
            KeyId::A,
            KeyId::M,
            KeyId::D0,
            KeyId::D8,
            KeyId::F1,
            KeyId::F10,
            KeyId::Space,
            KeyId::Enter,
            KeyId::Tab,
            KeyId::Backspace,
            KeyId::Escape,
            KeyId::LShift,
            KeyId::RShift,
            KeyId::LControl,
            KeyId::RControl,
            KeyId::LAlt,
            KeyId::RAlt,
        ];

        for key in keys {
            let rdev_key: Key = key.into();
            let parsed = KeyId::try_from(rdev_key).unwrap_or_else(|err| {
                panic!("expected round-trip conversion to succeed for {key:?}: {err}");
            });
            assert_eq!(parsed, key);
        }
    }

    #[test]
    fn test_key_mapping_mouse_button_conversions() {
        assert_eq!(KeyId::try_from(Button::Left), Ok(KeyId::Mouse1));
        assert_eq!(KeyId::try_from(Button::Right), Ok(KeyId::Mouse2));
        assert_eq!(KeyId::try_from(Button::Middle), Ok(KeyId::Mouse3));
        assert_eq!(KeyId::try_from(Button::Unknown(4)), Ok(KeyId::Mouse4));
        assert_eq!(KeyId::try_from(Button::Unknown(5)), Ok(KeyId::Mouse5));

        assert_eq!(rdev::Button::try_from(KeyId::Mouse1), Ok(Button::Left));
        assert_eq!(rdev::Button::try_from(KeyId::Mouse2), Ok(Button::Right));
        assert_eq!(rdev::Button::try_from(KeyId::Mouse3), Ok(Button::Middle));
        assert_eq!(
            rdev::Button::try_from(KeyId::Mouse4),
            Ok(Button::Unknown(4))
        );
        assert_eq!(
            rdev::Button::try_from(KeyId::Mouse5),
            Ok(Button::Unknown(5))
        );
    }
}
