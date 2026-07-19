//! Mapping from platform-native physical key codes to W3C UI Events `code`
//! values ("KeyA", "Digit2", "NumpadEnter", ...), independent of the active
//! keyboard layout.
//!
//! The table is derived from Chromium's `ui/events/keycodes/dom/dom_code_data.inc`
//! (BSD-style license, <https://source.chromium.org/chromium/chromium/src/+/main:ui/events/keycodes/dom/dom_code_data.inc>),
//! the canonical cross-platform mapping used by every major browser. Rows with
//! no W3C `code` name, or no mapping on any platform gpui serves, are omitted.
//!
//! Sentinels follow the source data: a `mac` value of `0xffff` and a `windows`
//! or `xkb` value of `0x0000` mean "no such key on that platform".

struct PhysicalKeyEntry {
    /// W3C UI Events `KeyboardEvent.code` value.
    code: &'static str,
    /// macOS virtual key code (`NSEvent.keyCode`); `0xffff` = absent.
    mac: u16,
    /// Windows Scan Code Set 1 value, `0xe0`-prefixed for extended keys
    /// (e.g. `NumpadEnter` = `0xe01c`); `0x0000` = absent.
    windows: u16,
    /// XKB keycode (evdev code + 8); `0x0000` = absent.
    xkb: u16,
}

const PHYSICAL_KEY_TABLE: &[PhysicalKeyEntry] = &[
    PhysicalKeyEntry { code: "Sleep", mac: 0xffff, windows: 0xe05f, xkb: 0x0096 },
    PhysicalKeyEntry { code: "WakeUp", mac: 0xffff, windows: 0xe063, xkb: 0x0097 },
    PhysicalKeyEntry { code: "DisplayToggleIntExt", mac: 0xffff, windows: 0x0000, xkb: 0x00eb },
    PhysicalKeyEntry { code: "KeyA", mac: 0x0000, windows: 0x001e, xkb: 0x0026 },
    PhysicalKeyEntry { code: "KeyB", mac: 0x000b, windows: 0x0030, xkb: 0x0038 },
    PhysicalKeyEntry { code: "KeyC", mac: 0x0008, windows: 0x002e, xkb: 0x0036 },
    PhysicalKeyEntry { code: "KeyD", mac: 0x0002, windows: 0x0020, xkb: 0x0028 },
    PhysicalKeyEntry { code: "KeyE", mac: 0x000e, windows: 0x0012, xkb: 0x001a },
    PhysicalKeyEntry { code: "KeyF", mac: 0x0003, windows: 0x0021, xkb: 0x0029 },
    PhysicalKeyEntry { code: "KeyG", mac: 0x0005, windows: 0x0022, xkb: 0x002a },
    PhysicalKeyEntry { code: "KeyH", mac: 0x0004, windows: 0x0023, xkb: 0x002b },
    PhysicalKeyEntry { code: "KeyI", mac: 0x0022, windows: 0x0017, xkb: 0x001f },
    PhysicalKeyEntry { code: "KeyJ", mac: 0x0026, windows: 0x0024, xkb: 0x002c },
    PhysicalKeyEntry { code: "KeyK", mac: 0x0028, windows: 0x0025, xkb: 0x002d },
    PhysicalKeyEntry { code: "KeyL", mac: 0x0025, windows: 0x0026, xkb: 0x002e },
    PhysicalKeyEntry { code: "KeyM", mac: 0x002e, windows: 0x0032, xkb: 0x003a },
    PhysicalKeyEntry { code: "KeyN", mac: 0x002d, windows: 0x0031, xkb: 0x0039 },
    PhysicalKeyEntry { code: "KeyO", mac: 0x001f, windows: 0x0018, xkb: 0x0020 },
    PhysicalKeyEntry { code: "KeyP", mac: 0x0023, windows: 0x0019, xkb: 0x0021 },
    PhysicalKeyEntry { code: "KeyQ", mac: 0x000c, windows: 0x0010, xkb: 0x0018 },
    PhysicalKeyEntry { code: "KeyR", mac: 0x000f, windows: 0x0013, xkb: 0x001b },
    PhysicalKeyEntry { code: "KeyS", mac: 0x0001, windows: 0x001f, xkb: 0x0027 },
    PhysicalKeyEntry { code: "KeyT", mac: 0x0011, windows: 0x0014, xkb: 0x001c },
    PhysicalKeyEntry { code: "KeyU", mac: 0x0020, windows: 0x0016, xkb: 0x001e },
    PhysicalKeyEntry { code: "KeyV", mac: 0x0009, windows: 0x002f, xkb: 0x0037 },
    PhysicalKeyEntry { code: "KeyW", mac: 0x000d, windows: 0x0011, xkb: 0x0019 },
    PhysicalKeyEntry { code: "KeyX", mac: 0x0007, windows: 0x002d, xkb: 0x0035 },
    PhysicalKeyEntry { code: "KeyY", mac: 0x0010, windows: 0x0015, xkb: 0x001d },
    PhysicalKeyEntry { code: "KeyZ", mac: 0x0006, windows: 0x002c, xkb: 0x0034 },
    PhysicalKeyEntry { code: "Digit1", mac: 0x0012, windows: 0x0002, xkb: 0x000a },
    PhysicalKeyEntry { code: "Digit2", mac: 0x0013, windows: 0x0003, xkb: 0x000b },
    PhysicalKeyEntry { code: "Digit3", mac: 0x0014, windows: 0x0004, xkb: 0x000c },
    PhysicalKeyEntry { code: "Digit4", mac: 0x0015, windows: 0x0005, xkb: 0x000d },
    PhysicalKeyEntry { code: "Digit5", mac: 0x0017, windows: 0x0006, xkb: 0x000e },
    PhysicalKeyEntry { code: "Digit6", mac: 0x0016, windows: 0x0007, xkb: 0x000f },
    PhysicalKeyEntry { code: "Digit7", mac: 0x001a, windows: 0x0008, xkb: 0x0010 },
    PhysicalKeyEntry { code: "Digit8", mac: 0x001c, windows: 0x0009, xkb: 0x0011 },
    PhysicalKeyEntry { code: "Digit9", mac: 0x0019, windows: 0x000a, xkb: 0x0012 },
    PhysicalKeyEntry { code: "Digit0", mac: 0x001d, windows: 0x000b, xkb: 0x0013 },
    PhysicalKeyEntry { code: "Enter", mac: 0x0024, windows: 0x001c, xkb: 0x0024 },
    PhysicalKeyEntry { code: "Escape", mac: 0x0035, windows: 0x0001, xkb: 0x0009 },
    PhysicalKeyEntry { code: "Backspace", mac: 0x0033, windows: 0x000e, xkb: 0x0016 },
    PhysicalKeyEntry { code: "Tab", mac: 0x0030, windows: 0x000f, xkb: 0x0017 },
    PhysicalKeyEntry { code: "Space", mac: 0x0031, windows: 0x0039, xkb: 0x0041 },
    PhysicalKeyEntry { code: "Minus", mac: 0x001b, windows: 0x000c, xkb: 0x0014 },
    PhysicalKeyEntry { code: "Equal", mac: 0x0018, windows: 0x000d, xkb: 0x0015 },
    PhysicalKeyEntry { code: "BracketLeft", mac: 0x0021, windows: 0x001a, xkb: 0x0022 },
    PhysicalKeyEntry { code: "BracketRight", mac: 0x001e, windows: 0x001b, xkb: 0x0023 },
    PhysicalKeyEntry { code: "Backslash", mac: 0x002a, windows: 0x002b, xkb: 0x0033 },
    PhysicalKeyEntry { code: "Semicolon", mac: 0x0029, windows: 0x0027, xkb: 0x002f },
    PhysicalKeyEntry { code: "Quote", mac: 0x0027, windows: 0x0028, xkb: 0x0030 },
    PhysicalKeyEntry { code: "Backquote", mac: 0x0032, windows: 0x0029, xkb: 0x0031 },
    PhysicalKeyEntry { code: "Comma", mac: 0x002b, windows: 0x0033, xkb: 0x003b },
    PhysicalKeyEntry { code: "Period", mac: 0x002f, windows: 0x0034, xkb: 0x003c },
    PhysicalKeyEntry { code: "Slash", mac: 0x002c, windows: 0x0035, xkb: 0x003d },
    PhysicalKeyEntry { code: "CapsLock", mac: 0x0039, windows: 0x003a, xkb: 0x0042 },
    PhysicalKeyEntry { code: "F1", mac: 0x007a, windows: 0x003b, xkb: 0x0043 },
    PhysicalKeyEntry { code: "F2", mac: 0x0078, windows: 0x003c, xkb: 0x0044 },
    PhysicalKeyEntry { code: "F3", mac: 0x0063, windows: 0x003d, xkb: 0x0045 },
    PhysicalKeyEntry { code: "F4", mac: 0x0076, windows: 0x003e, xkb: 0x0046 },
    PhysicalKeyEntry { code: "F5", mac: 0x0060, windows: 0x003f, xkb: 0x0047 },
    PhysicalKeyEntry { code: "F6", mac: 0x0061, windows: 0x0040, xkb: 0x0048 },
    PhysicalKeyEntry { code: "F7", mac: 0x0062, windows: 0x0041, xkb: 0x0049 },
    PhysicalKeyEntry { code: "F8", mac: 0x0064, windows: 0x0042, xkb: 0x004a },
    PhysicalKeyEntry { code: "F9", mac: 0x0065, windows: 0x0043, xkb: 0x004b },
    PhysicalKeyEntry { code: "F10", mac: 0x006d, windows: 0x0044, xkb: 0x004c },
    PhysicalKeyEntry { code: "F11", mac: 0x0067, windows: 0x0057, xkb: 0x005f },
    PhysicalKeyEntry { code: "F12", mac: 0x006f, windows: 0x0058, xkb: 0x0060 },
    PhysicalKeyEntry { code: "PrintScreen", mac: 0xffff, windows: 0xe037, xkb: 0x006b },
    PhysicalKeyEntry { code: "ScrollLock", mac: 0xffff, windows: 0x0046, xkb: 0x004e },
    PhysicalKeyEntry { code: "Pause", mac: 0xffff, windows: 0x0045, xkb: 0x007f },
    PhysicalKeyEntry { code: "Insert", mac: 0x0072, windows: 0xe052, xkb: 0x0076 },
    PhysicalKeyEntry { code: "Home", mac: 0x0073, windows: 0xe047, xkb: 0x006e },
    PhysicalKeyEntry { code: "PageUp", mac: 0x0074, windows: 0xe049, xkb: 0x0070 },
    PhysicalKeyEntry { code: "Delete", mac: 0x0075, windows: 0xe053, xkb: 0x0077 },
    PhysicalKeyEntry { code: "End", mac: 0x0077, windows: 0xe04f, xkb: 0x0073 },
    PhysicalKeyEntry { code: "PageDown", mac: 0x0079, windows: 0xe051, xkb: 0x0075 },
    PhysicalKeyEntry { code: "ArrowRight", mac: 0x007c, windows: 0xe04d, xkb: 0x0072 },
    PhysicalKeyEntry { code: "ArrowLeft", mac: 0x007b, windows: 0xe04b, xkb: 0x0071 },
    PhysicalKeyEntry { code: "ArrowDown", mac: 0x007d, windows: 0xe050, xkb: 0x0074 },
    PhysicalKeyEntry { code: "ArrowUp", mac: 0x007e, windows: 0xe048, xkb: 0x006f },
    PhysicalKeyEntry { code: "NumLock", mac: 0x0047, windows: 0xe045, xkb: 0x004d },
    PhysicalKeyEntry { code: "NumpadDivide", mac: 0x004b, windows: 0xe035, xkb: 0x006a },
    PhysicalKeyEntry { code: "NumpadMultiply", mac: 0x0043, windows: 0x0037, xkb: 0x003f },
    PhysicalKeyEntry { code: "NumpadSubtract", mac: 0x004e, windows: 0x004a, xkb: 0x0052 },
    PhysicalKeyEntry { code: "NumpadAdd", mac: 0x0045, windows: 0x004e, xkb: 0x0056 },
    PhysicalKeyEntry { code: "NumpadEnter", mac: 0x004c, windows: 0xe01c, xkb: 0x0068 },
    PhysicalKeyEntry { code: "Numpad1", mac: 0x0053, windows: 0x004f, xkb: 0x0057 },
    PhysicalKeyEntry { code: "Numpad2", mac: 0x0054, windows: 0x0050, xkb: 0x0058 },
    PhysicalKeyEntry { code: "Numpad3", mac: 0x0055, windows: 0x0051, xkb: 0x0059 },
    PhysicalKeyEntry { code: "Numpad4", mac: 0x0056, windows: 0x004b, xkb: 0x0053 },
    PhysicalKeyEntry { code: "Numpad5", mac: 0x0057, windows: 0x004c, xkb: 0x0054 },
    PhysicalKeyEntry { code: "Numpad6", mac: 0x0058, windows: 0x004d, xkb: 0x0055 },
    PhysicalKeyEntry { code: "Numpad7", mac: 0x0059, windows: 0x0047, xkb: 0x004f },
    PhysicalKeyEntry { code: "Numpad8", mac: 0x005b, windows: 0x0048, xkb: 0x0050 },
    PhysicalKeyEntry { code: "Numpad9", mac: 0x005c, windows: 0x0049, xkb: 0x0051 },
    PhysicalKeyEntry { code: "Numpad0", mac: 0x0052, windows: 0x0052, xkb: 0x005a },
    PhysicalKeyEntry { code: "NumpadDecimal", mac: 0x0041, windows: 0x0053, xkb: 0x005b },
    PhysicalKeyEntry { code: "IntlBackslash", mac: 0x000a, windows: 0x0056, xkb: 0x005e },
    PhysicalKeyEntry { code: "ContextMenu", mac: 0x006e, windows: 0xe05d, xkb: 0x0087 },
    PhysicalKeyEntry { code: "Power", mac: 0xffff, windows: 0xe05e, xkb: 0x007c },
    PhysicalKeyEntry { code: "NumpadEqual", mac: 0x0051, windows: 0x0059, xkb: 0x007d },
    PhysicalKeyEntry { code: "F13", mac: 0x0069, windows: 0x0064, xkb: 0x00bf },
    PhysicalKeyEntry { code: "F14", mac: 0x006b, windows: 0x0065, xkb: 0x00c0 },
    PhysicalKeyEntry { code: "F15", mac: 0x0071, windows: 0x0066, xkb: 0x00c1 },
    PhysicalKeyEntry { code: "F16", mac: 0x006a, windows: 0x0067, xkb: 0x00c2 },
    PhysicalKeyEntry { code: "F17", mac: 0x0040, windows: 0x0068, xkb: 0x00c3 },
    PhysicalKeyEntry { code: "F18", mac: 0x004f, windows: 0x0069, xkb: 0x00c4 },
    PhysicalKeyEntry { code: "F19", mac: 0x0050, windows: 0x006a, xkb: 0x00c5 },
    PhysicalKeyEntry { code: "F20", mac: 0x005a, windows: 0x006b, xkb: 0x00c6 },
    PhysicalKeyEntry { code: "F21", mac: 0xffff, windows: 0x006c, xkb: 0x00c7 },
    PhysicalKeyEntry { code: "F22", mac: 0xffff, windows: 0x006d, xkb: 0x00c8 },
    PhysicalKeyEntry { code: "F23", mac: 0xffff, windows: 0x006e, xkb: 0x00c9 },
    PhysicalKeyEntry { code: "F24", mac: 0xffff, windows: 0x0076, xkb: 0x00ca },
    PhysicalKeyEntry { code: "Open", mac: 0xffff, windows: 0x0000, xkb: 0x008e },
    PhysicalKeyEntry { code: "Help", mac: 0xffff, windows: 0xe03b, xkb: 0x0092 },
    PhysicalKeyEntry { code: "Select", mac: 0xffff, windows: 0x0000, xkb: 0x008c },
    PhysicalKeyEntry { code: "Again", mac: 0xffff, windows: 0x0000, xkb: 0x0089 },
    PhysicalKeyEntry { code: "Undo", mac: 0xffff, windows: 0xe008, xkb: 0x008b },
    PhysicalKeyEntry { code: "Cut", mac: 0xffff, windows: 0xe017, xkb: 0x0091 },
    PhysicalKeyEntry { code: "Copy", mac: 0xffff, windows: 0xe018, xkb: 0x008d },
    PhysicalKeyEntry { code: "Paste", mac: 0xffff, windows: 0xe00a, xkb: 0x008f },
    PhysicalKeyEntry { code: "Find", mac: 0xffff, windows: 0x0000, xkb: 0x0090 },
    PhysicalKeyEntry { code: "AudioVolumeMute", mac: 0x004a, windows: 0xe020, xkb: 0x0079 },
    PhysicalKeyEntry { code: "AudioVolumeUp", mac: 0x0048, windows: 0xe030, xkb: 0x007b },
    PhysicalKeyEntry { code: "AudioVolumeDown", mac: 0x0049, windows: 0xe02e, xkb: 0x007a },
    PhysicalKeyEntry { code: "NumpadComma", mac: 0x005f, windows: 0x007e, xkb: 0x0081 },
    PhysicalKeyEntry { code: "IntlRo", mac: 0x005e, windows: 0x0073, xkb: 0x0061 },
    PhysicalKeyEntry { code: "KanaMode", mac: 0xffff, windows: 0x0070, xkb: 0x0065 },
    PhysicalKeyEntry { code: "IntlYen", mac: 0x005d, windows: 0x007d, xkb: 0x0084 },
    PhysicalKeyEntry { code: "Convert", mac: 0xffff, windows: 0x0079, xkb: 0x0064 },
    PhysicalKeyEntry { code: "NonConvert", mac: 0xffff, windows: 0x007b, xkb: 0x0066 },
    PhysicalKeyEntry { code: "Lang1", mac: 0x0068, windows: 0x0072, xkb: 0x0082 },
    PhysicalKeyEntry { code: "Lang2", mac: 0x0066, windows: 0x0071, xkb: 0x0083 },
    PhysicalKeyEntry { code: "Lang3", mac: 0xffff, windows: 0x0078, xkb: 0x0062 },
    PhysicalKeyEntry { code: "Lang4", mac: 0xffff, windows: 0x0077, xkb: 0x0063 },
    PhysicalKeyEntry { code: "Lang5", mac: 0xffff, windows: 0x0000, xkb: 0x005d },
    PhysicalKeyEntry { code: "NumpadParenLeft", mac: 0xffff, windows: 0x0000, xkb: 0x00bb },
    PhysicalKeyEntry { code: "NumpadParenRight", mac: 0xffff, windows: 0x0000, xkb: 0x00bc },
    PhysicalKeyEntry { code: "ControlLeft", mac: 0x003b, windows: 0x001d, xkb: 0x0025 },
    PhysicalKeyEntry { code: "ShiftLeft", mac: 0x0038, windows: 0x002a, xkb: 0x0032 },
    PhysicalKeyEntry { code: "AltLeft", mac: 0x003a, windows: 0x0038, xkb: 0x0040 },
    PhysicalKeyEntry { code: "MetaLeft", mac: 0x0037, windows: 0xe05b, xkb: 0x0085 },
    PhysicalKeyEntry { code: "ControlRight", mac: 0x003e, windows: 0xe01d, xkb: 0x0069 },
    PhysicalKeyEntry { code: "ShiftRight", mac: 0x003c, windows: 0x0036, xkb: 0x003e },
    PhysicalKeyEntry { code: "AltRight", mac: 0x003d, windows: 0xe038, xkb: 0x006c },
    PhysicalKeyEntry { code: "MetaRight", mac: 0x0036, windows: 0xe05c, xkb: 0x0086 },
    PhysicalKeyEntry { code: "BrightnessUp", mac: 0xffff, windows: 0x0000, xkb: 0x00e9 },
    PhysicalKeyEntry { code: "BrightnessDown", mac: 0xffff, windows: 0x0000, xkb: 0x00e8 },
    PhysicalKeyEntry { code: "MediaPlay", mac: 0xffff, windows: 0x0000, xkb: 0x00d7 },
    PhysicalKeyEntry { code: "MediaPause", mac: 0xffff, windows: 0x0000, xkb: 0x00d1 },
    PhysicalKeyEntry { code: "MediaRecord", mac: 0xffff, windows: 0x0000, xkb: 0x00af },
    PhysicalKeyEntry { code: "MediaFastForward", mac: 0xffff, windows: 0x0000, xkb: 0x00d8 },
    PhysicalKeyEntry { code: "MediaRewind", mac: 0xffff, windows: 0x0000, xkb: 0x00b0 },
    PhysicalKeyEntry { code: "MediaTrackNext", mac: 0xffff, windows: 0xe019, xkb: 0x00ab },
    PhysicalKeyEntry { code: "MediaTrackPrevious", mac: 0xffff, windows: 0xe010, xkb: 0x00ad },
    PhysicalKeyEntry { code: "MediaStop", mac: 0xffff, windows: 0xe024, xkb: 0x00ae },
    PhysicalKeyEntry { code: "Eject", mac: 0xffff, windows: 0xe02c, xkb: 0x00a9 },
    PhysicalKeyEntry { code: "MediaPlayPause", mac: 0xffff, windows: 0xe022, xkb: 0x00ac },
    PhysicalKeyEntry { code: "MediaSelect", mac: 0xffff, windows: 0xe06d, xkb: 0x00b3 },
    PhysicalKeyEntry { code: "LaunchMail", mac: 0xffff, windows: 0xe06c, xkb: 0x00a3 },
    PhysicalKeyEntry { code: "LaunchApp2", mac: 0xffff, windows: 0xe021, xkb: 0x0094 },
    PhysicalKeyEntry { code: "LaunchApp1", mac: 0xffff, windows: 0xe06b, xkb: 0x0098 },
    PhysicalKeyEntry { code: "LaunchControlPanel", mac: 0xffff, windows: 0x0000, xkb: 0x024b },
    PhysicalKeyEntry { code: "SelectTask", mac: 0xffff, windows: 0x0000, xkb: 0x024c },
    PhysicalKeyEntry { code: "LaunchScreenSaver", mac: 0xffff, windows: 0x0000, xkb: 0x024d },
    PhysicalKeyEntry { code: "LaunchAssistant", mac: 0xffff, windows: 0x0000, xkb: 0x024f },
    PhysicalKeyEntry { code: "BrowserSearch", mac: 0xffff, windows: 0xe065, xkb: 0x00e1 },
    PhysicalKeyEntry { code: "BrowserHome", mac: 0xffff, windows: 0xe032, xkb: 0x00b4 },
    PhysicalKeyEntry { code: "BrowserBack", mac: 0xffff, windows: 0xe06a, xkb: 0x00a6 },
    PhysicalKeyEntry { code: "BrowserForward", mac: 0xffff, windows: 0xe069, xkb: 0x00a7 },
    PhysicalKeyEntry { code: "BrowserStop", mac: 0xffff, windows: 0xe068, xkb: 0x0088 },
    PhysicalKeyEntry { code: "BrowserRefresh", mac: 0xffff, windows: 0xe067, xkb: 0x00b5 },
    PhysicalKeyEntry { code: "BrowserFavorites", mac: 0xffff, windows: 0xe066, xkb: 0x00a4 },
    PhysicalKeyEntry { code: "ZoomToggle", mac: 0xffff, windows: 0x0000, xkb: 0x017c },
    PhysicalKeyEntry { code: "MailReply", mac: 0xffff, windows: 0x0000, xkb: 0x00f0 },
    PhysicalKeyEntry { code: "MailForward", mac: 0xffff, windows: 0x0000, xkb: 0x00f1 },
    PhysicalKeyEntry { code: "MailSend", mac: 0xffff, windows: 0x0000, xkb: 0x00ef },
    PhysicalKeyEntry { code: "KeyboardLayoutSelect", mac: 0xffff, windows: 0x0000, xkb: 0x0250 },
    PhysicalKeyEntry { code: "ShowAllWindows", mac: 0xffff, windows: 0x0000, xkb: 0x0080 },
];

/// Returns the W3C UI Events `code` for a macOS virtual key code
/// (`NSEvent.keyCode`), if known.
pub fn physical_key_from_macos_keycode(keycode: u16) -> Option<&'static str> {
    PHYSICAL_KEY_TABLE
        .iter()
        .find(|entry| entry.mac != 0xffff && entry.mac == keycode)
        .map(|entry| entry.code)
}

/// Returns the W3C UI Events `code` for a Windows Scan Code Set 1 value, if
/// known. Extended keys must be passed `0xe0`-prefixed (`0xe000 | scan_code`).
pub fn physical_key_from_windows_scancode(scancode: u16) -> Option<&'static str> {
    PHYSICAL_KEY_TABLE
        .iter()
        .find(|entry| entry.windows != 0x0000 && entry.windows == scancode)
        .map(|entry| entry.code)
}

/// Returns the W3C UI Events `code` for an XKB keycode (evdev code + 8), if
/// known.
pub fn physical_key_from_xkb_keycode(keycode: u32) -> Option<&'static str> {
    let keycode = u16::try_from(keycode).ok()?;
    PHYSICAL_KEY_TABLE
        .iter()
        .find(|entry| entry.xkb != 0x0000 && entry.xkb == keycode)
        .map(|entry| entry.code)
}

/// Returns whether `code` is a W3C UI Events `code` value this table knows on
/// at least one platform. Useful for validating user-supplied physical-key
/// names at configuration time.
pub fn is_known_physical_key_code(code: &str) -> bool {
    PHYSICAL_KEY_TABLE.iter().any(|entry| entry.code == code)
}

/// Iterates every W3C UI Events `code` value known to this table.
pub fn physical_key_codes() -> impl Iterator<Item = &'static str> {
    PHYSICAL_KEY_TABLE.iter().map(|entry| entry.code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicate_codes() {
        let mut seen = std::collections::HashSet::new();
        for entry in PHYSICAL_KEY_TABLE {
            assert!(seen.insert(entry.code), "duplicate code {}", entry.code);
        }
    }

    #[test]
    fn no_duplicate_native_codes_per_platform() {
        let mut mac = std::collections::HashSet::new();
        let mut windows = std::collections::HashSet::new();
        let mut xkb = std::collections::HashSet::new();
        for entry in PHYSICAL_KEY_TABLE {
            if entry.mac != 0xffff {
                assert!(mac.insert(entry.mac), "duplicate mac code {:#x}", entry.mac);
            }
            if entry.windows != 0x0000 {
                assert!(
                    windows.insert(entry.windows),
                    "duplicate windows scancode {:#x}",
                    entry.windows
                );
            }
            if entry.xkb != 0x0000 {
                assert!(xkb.insert(entry.xkb), "duplicate xkb code {:#x}", entry.xkb);
            }
        }
    }

    #[test]
    fn well_known_rows_spot_check() {
        // Values cross-checked against Chromium's dom_code_data.inc.
        assert_eq!(physical_key_from_macos_keycode(0x00), Some("KeyA"));
        assert_eq!(physical_key_from_macos_keycode(0x24), Some("Enter"));
        assert_eq!(physical_key_from_macos_keycode(0x35), Some("Escape"));
        assert_eq!(physical_key_from_windows_scancode(0x001e), Some("KeyA"));
        assert_eq!(physical_key_from_windows_scancode(0xe01c), Some("NumpadEnter"));
        assert_eq!(physical_key_from_xkb_keycode(0x26), Some("KeyA"));
        assert_eq!(physical_key_from_xkb_keycode(0x09), Some("Escape"));
    }

    #[test]
    fn sentinels_do_not_match() {
        // 0xffff (mac) and 0x0000 (windows/xkb) are absence markers, never keys.
        assert_eq!(physical_key_from_macos_keycode(0xffff), None);
        assert_eq!(physical_key_from_windows_scancode(0x0000), None);
        assert_eq!(physical_key_from_xkb_keycode(0x0000), None);
    }

    #[test]
    fn validation_helpers_agree_with_table() {
        assert!(is_known_physical_key_code("KeyA"));
        assert!(is_known_physical_key_code("BracketLeft"));
        assert!(!is_known_physical_key_code("KeyÄ"));
        assert!(!is_known_physical_key_code("keya"));
        assert_eq!(physical_key_codes().count(), PHYSICAL_KEY_TABLE.len());
    }
}
