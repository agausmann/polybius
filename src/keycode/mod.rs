pub mod qmk;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keycode {
    Hid(HidKeycode),
    System(SystemKeycode),
    User(u8),
}

impl From<SystemKeycode> for Keycode {
    fn from(v: SystemKeycode) -> Self {
        Self::System(v)
    }
}

impl From<HidKeycode> for Keycode {
    fn from(v: HidKeycode) -> Self {
        Self::Hid(v)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeycode {
    None,
    Transparent,
}

/// Keycodes from the USB HID Usage Tables, Keyboard/Keypad Page (0x07).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HidKeycode {
    ErrorRollOver = 0x01,
    PostFail,
    ErrorUndefined,
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

    // 0x10
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
    Num1,
    Num2,

    // 0x20
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equal,
    LeftBracket,

    // 0x30
    RightBracket,
    Backslash,
    NonUsHash,
    Semicolon,
    Quote,
    Grave,
    Comma,
    Dot,
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,

    // 0x40
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,

    // 0x50
    Left,
    Down,
    Up,
    NumLock,
    KeypadSlash,
    KeypadAsterisk,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,

    // 0x60
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadDot,
    NonUsBackslash,
    Application,
    Power,
    KeypadEqual,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,

    // 0x70
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,

    // 0x80
    VolumeUp,
    VolumeDown,
    LockingCaps,
    LockingNum,
    LockingScroll,
    KeypadComma,
    KeypadEqualAS400,
    Int1,
    Int2,
    Int3,
    Int4,
    Int5,
    Int6,
    Int7,
    Int8,
    Int9,

    // 0x90
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AltErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,

    // 0xA0
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // A5-AF Reserved

    // 0xB0
    Keypad00 = 0xb0,
    Keypad000,
    ThousandsSep,
    DecimalSep,
    CurrencyUnit,
    CurrencySubUnit,
    KeypadLeftParen,
    KeypadRightParen,
    KeypadLeftCBracket,
    KeypadRightCBracket,
    KeypadTab,
    KeypadBackspace,
    KeypadA,
    KeypadB,
    KeypadC,
    KeypadD,

    // 0xC0
    KeypadE,
    KeypadF,
    KeypadXor,
    KeypadHat,
    KeypadPercent,
    KeypadLess,
    KeypadGreater,
    KeypadAnd,
    KeypadLazyAnd,
    KeypadOr,
    KeypadLazyOr,
    KeypadColon,
    KeypadHash,
    KeypadSpace,
    KeypadAt,
    KeypadExclamation,

    // 0xD0
    KeypadMemStore,
    KeypadMemRecall,
    KeypadMemClear,
    KeypadMemAdd,
    KeypadMemSub,
    KeypadMemMul,
    KeypadMemDiv,
    KeypadPlusMinus,
    KeypadClear,
    KeypadClearEntry,
    KeypadBinary,
    KeypadOctal,
    KeypadDecimal,
    KeypadHexadecimal,

    // DE-DF Reserved

    // 0xE0
    LeftControl = 0xe0,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightControl,
    RightShift,
    RightAlt,
    RightGui,
}
