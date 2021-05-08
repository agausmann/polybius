/// Keycodes from the USB HID Usage Tables, Keyboard/Keypad Page (0x07).
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Keycode {
    // 0x00
    None,
    ErrorRollOver,
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

pub mod abbrev {
    //! Abbreviated keycode constants, similar to QMK and TMK, that are more
    //! convenient for writing keymaps.
    //!
    //! Intended usage of this module is to be glob-imported like a prelude:
    //!
    //! ```
    //! use kbforge::keycode::abbrev::*;
    //! ```
    //!
    //! All of the constants are prefixed with `KC_`, so there is little risk
    //! of conflicting imports.

    use super::Keycode;

    pub const KC_NO: Keycode = Keycode::None;
    pub const KC_A: Keycode = Keycode::A;
    pub const KC_B: Keycode = Keycode::B;
    pub const KC_C: Keycode = Keycode::C;
    pub const KC_D: Keycode = Keycode::D;
    pub const KC_F: Keycode = Keycode::F;
    pub const KC_G: Keycode = Keycode::G;
    pub const KC_H: Keycode = Keycode::H;
    pub const KC_I: Keycode = Keycode::I;
    pub const KC_J: Keycode = Keycode::J;
    pub const KC_K: Keycode = Keycode::K;
    pub const KC_L: Keycode = Keycode::L;
    pub const KC_M: Keycode = Keycode::M;
    pub const KC_N: Keycode = Keycode::N;
    pub const KC_O: Keycode = Keycode::O;
    pub const KC_P: Keycode = Keycode::P;
    pub const KC_Q: Keycode = Keycode::Q;
    pub const KC_R: Keycode = Keycode::R;
    pub const KC_S: Keycode = Keycode::S;
    pub const KC_T: Keycode = Keycode::T;
    pub const KC_U: Keycode = Keycode::U;
    pub const KC_V: Keycode = Keycode::V;
    pub const KC_W: Keycode = Keycode::W;
    pub const KC_X: Keycode = Keycode::X;
    pub const KC_Y: Keycode = Keycode::Y;
    pub const KC_Z: Keycode = Keycode::Z;
    pub const KC_1: Keycode = Keycode::Num1;
    pub const KC_2: Keycode = Keycode::Num2;
    pub const KC_3: Keycode = Keycode::Num3;
    pub const KC_4: Keycode = Keycode::Num4;
    pub const KC_5: Keycode = Keycode::Num5;
    pub const KC_6: Keycode = Keycode::Num6;
    pub const KC_7: Keycode = Keycode::Num7;
    pub const KC_8: Keycode = Keycode::Num8;
    pub const KC_9: Keycode = Keycode::Num9;
    pub const KC_0: Keycode = Keycode::Num0;
    pub const KC_ENT: Keycode = Keycode::Enter;
    pub const KC_ESC: Keycode = Keycode::Escape;
    pub const KC_BSPC: Keycode = Keycode::Backspace;
    pub const KC_TAB: Keycode = Keycode::Tab;
    pub const KC_SPC: Keycode = Keycode::Space;
    pub const KC_MINS: Keycode = Keycode::Minus;
    pub const KC_EQL: Keycode = Keycode::Equal;
    pub const KC_LBRC: Keycode = Keycode::LeftBracket;
    pub const KC_RBRC: Keycode = Keycode::RightBracket;
    pub const KC_BSLS: Keycode = Keycode::Backslash;
    pub const KC_NUHS: Keycode = Keycode::NonUsHash;
    pub const KC_SCLN: Keycode = Keycode::Semicolon;
    pub const KC_QUOT: Keycode = Keycode::Quote;
    pub const KC_GRV: Keycode = Keycode::Grave;
    pub const KC_COMM: Keycode = Keycode::Comma;
    pub const KC_DOT: Keycode = Keycode::Dot;
    pub const KC_SLSH: Keycode = Keycode::Slash;
    pub const KC_CLCK: Keycode = Keycode::CapsLock;
    pub const KC_F1: Keycode = Keycode::F1;
    pub const KC_F2: Keycode = Keycode::F2;
    pub const KC_F3: Keycode = Keycode::F3;
    pub const KC_F4: Keycode = Keycode::F4;
    pub const KC_F5: Keycode = Keycode::F5;
    pub const KC_F6: Keycode = Keycode::F6;
    pub const KC_F7: Keycode = Keycode::F7;
    pub const KC_F8: Keycode = Keycode::F8;
    pub const KC_F9: Keycode = Keycode::F9;
    pub const KC_F10: Keycode = Keycode::F10;
    pub const KC_F11: Keycode = Keycode::F11;
    pub const KC_F12: Keycode = Keycode::F12;
    pub const KC_PSCR: Keycode = Keycode::PrintScreen;
    pub const KC_SLCK: Keycode = Keycode::ScrollLock;
    pub const KC_PAUS: Keycode = Keycode::Pause;
    pub const KC_INS: Keycode = Keycode::Insert;
    pub const KC_HOME: Keycode = Keycode::Home;
    pub const KC_PGUP: Keycode = Keycode::PageUp;
    pub const KC_DEL: Keycode = Keycode::Delete;
    pub const KC_END: Keycode = Keycode::End;
    pub const KC_PGDN: Keycode = Keycode::PageDown;
    pub const KC_RGHT: Keycode = Keycode::Right;
    pub const KC_LEFT: Keycode = Keycode::Left;
    pub const KC_DOWN: Keycode = Keycode::Down;
    pub const KC_UP: Keycode = Keycode::Up;
    pub const KC_NLCK: Keycode = Keycode::NumLock;
    pub const KC_PSLS: Keycode = Keycode::KeypadSlash;
    pub const KC_PAST: Keycode = Keycode::KeypadAsterisk;
    pub const KC_PMNS: Keycode = Keycode::KeypadMinus;
    pub const KC_PPLS: Keycode = Keycode::KeypadPlus;
    pub const KC_PENT: Keycode = Keycode::KeypadEnter;
    pub const KC_P1: Keycode = Keycode::Keypad1;
    pub const KC_P2: Keycode = Keycode::Keypad2;
    pub const KC_P3: Keycode = Keycode::Keypad3;
    pub const KC_P4: Keycode = Keycode::Keypad4;
    pub const KC_P5: Keycode = Keycode::Keypad5;
    pub const KC_P6: Keycode = Keycode::Keypad6;
    pub const KC_P7: Keycode = Keycode::Keypad7;
    pub const KC_P8: Keycode = Keycode::Keypad8;
    pub const KC_P9: Keycode = Keycode::Keypad9;
    pub const KC_P0: Keycode = Keycode::Keypad0;
    pub const KC_PDOT: Keycode = Keycode::KeypadDot;
    pub const KC_NUBS: Keycode = Keycode::NonUsBackslash;
    pub const KC_APP: Keycode = Keycode::Application;
    pub const KC_POWER: Keycode = Keycode::Power;
    pub const KC_PEQL: Keycode = Keycode::KeypadEqual;
    pub const KC_F13: Keycode = Keycode::F13;
    pub const KC_F14: Keycode = Keycode::F14;
    pub const KC_F15: Keycode = Keycode::F15;
    pub const KC_F16: Keycode = Keycode::F16;
    pub const KC_F17: Keycode = Keycode::F17;
    pub const KC_F18: Keycode = Keycode::F18;
    pub const KC_F19: Keycode = Keycode::F19;
    pub const KC_F20: Keycode = Keycode::F20;
    pub const KC_F21: Keycode = Keycode::F21;
    pub const KC_F22: Keycode = Keycode::F22;
    pub const KC_F23: Keycode = Keycode::F23;
    pub const KC_F24: Keycode = Keycode::F24;
    pub const KC_EXEC: Keycode = Keycode::Execute;
    pub const KC_HELP: Keycode = Keycode::Help;
    pub const KC_MENU: Keycode = Keycode::Menu;
    pub const KC_SLCT: Keycode = Keycode::Select;
    pub const KC_STOP: Keycode = Keycode::Stop;
    pub const KC_AGIN: Keycode = Keycode::Again;
    pub const KC_UNDO: Keycode = Keycode::Undo;
    pub const KC_CUT: Keycode = Keycode::Cut;
    pub const KC_COPY: Keycode = Keycode::Copy;
    pub const KC_PSTE: Keycode = Keycode::Paste;
    pub const KC_FIND: Keycode = Keycode::Find;
    pub const KC_MUTE: Keycode = Keycode::Mute;
    pub const KC_VOLU: Keycode = Keycode::VolumeUp;
    pub const KC_VOLD: Keycode = Keycode::VolumeDown;
    pub const KC_LCAP: Keycode = Keycode::LockingCaps;
    pub const KC_LNUM: Keycode = Keycode::LockingNum;
    pub const KC_LSCR: Keycode = Keycode::LockingScroll;
    pub const KC_PCMM: Keycode = Keycode::KeypadComma;
    pub const KC_KP_EQUAL_AS400: Keycode = Keycode::KeypadEqualAS400;
    pub const KC_INT1: Keycode = Keycode::Int1;
    pub const KC_INT2: Keycode = Keycode::Int2;
    pub const KC_INT3: Keycode = Keycode::Int3;
    pub const KC_INT4: Keycode = Keycode::Int4;
    pub const KC_INT5: Keycode = Keycode::Int5;
    pub const KC_INT6: Keycode = Keycode::Int6;
    pub const KC_INT7: Keycode = Keycode::Int7;
    pub const KC_INT8: Keycode = Keycode::Int8;
    pub const KC_INT9: Keycode = Keycode::Int9;
    pub const KC_LANG1: Keycode = Keycode::Lang1;
    pub const KC_LANG2: Keycode = Keycode::Lang2;
    pub const KC_LANG3: Keycode = Keycode::Lang3;
    pub const KC_LANG4: Keycode = Keycode::Lang4;
    pub const KC_LANG5: Keycode = Keycode::Lang5;
    pub const KC_LANG6: Keycode = Keycode::Lang6;
    pub const KC_LANG7: Keycode = Keycode::Lang7;
    pub const KC_LANG8: Keycode = Keycode::Lang8;
    pub const KC_LANG9: Keycode = Keycode::Lang9;
    pub const KC_ERAS: Keycode = Keycode::AltErase;
    pub const KC_SYSREQ: Keycode = Keycode::SysReq;
    pub const KC_CANCEL: Keycode = Keycode::Cancel;
    pub const KC_CLR: Keycode = Keycode::Clear;
    pub const KC_PRIOR: Keycode = Keycode::Prior;
    pub const KC_RETURN: Keycode = Keycode::Return;
    pub const KC_SEPARATOR: Keycode = Keycode::Separator;
    pub const KC_OUT: Keycode = Keycode::Out;
    pub const KC_OPER: Keycode = Keycode::Oper;
    pub const KC_CLEAR_AGAIN: Keycode = Keycode::ClearAgain;
    pub const KC_CRSEL: Keycode = Keycode::CrSel;
    pub const KC_EXSEL: Keycode = Keycode::ExSel;
    pub const KC_P00: Keycode = Keycode::Keypad00;
    pub const KC_P000: Keycode = Keycode::Keypad000;
    pub const KC_THOUSANDS_SEPARATOR: Keycode = Keycode::ThousandsSep;
    pub const KC_DECIMAL_SEPARATOR: Keycode = Keycode::DecimalSep;
    pub const KC_CURRENCY_UNIT: Keycode = Keycode::CurrencyUnit;
    pub const KC_CURRENCY_SUB_UNIT: Keycode = Keycode::CurrencySubUnit;
    pub const KC_KP_LPAREN: Keycode = Keycode::KeypadLeftParen;
    pub const KC_KP_RPAREN: Keycode = Keycode::KeypadRightParen;
    pub const KC_KP_LCBRACKET: Keycode = Keycode::KeypadLeftCBracket;
    pub const KC_KP_RCBRACKET: Keycode = Keycode::KeypadRightCBracket;
    pub const KC_KP_TAB: Keycode = Keycode::KeypadTab;
    pub const KC_KP_BSPACE: Keycode = Keycode::KeypadBackspace;
    pub const KC_KP_A: Keycode = Keycode::KeypadA;
    pub const KC_KP_B: Keycode = Keycode::KeypadB;
    pub const KC_KP_C: Keycode = Keycode::KeypadC;
    pub const KC_KP_D: Keycode = Keycode::KeypadD;
    pub const KC_KP_E: Keycode = Keycode::KeypadE;
    pub const KC_KP_F: Keycode = Keycode::KeypadF;
    pub const KC_KP_XOR: Keycode = Keycode::KeypadXor;
    pub const KC_KP_HAT: Keycode = Keycode::KeypadHat;
    pub const KC_KP_PERC: Keycode = Keycode::KeypadPercent;
    pub const KC_KP_LT: Keycode = Keycode::KeypadLess;
    pub const KC_KP_GT: Keycode = Keycode::KeypadGreater;
    pub const KC_KP_AND: Keycode = Keycode::KeypadAnd;
    pub const KC_KP_LAZYAND: Keycode = Keycode::KeypadLazyAnd;
    pub const KC_KP_OR: Keycode = Keycode::KeypadOr;
    pub const KC_KP_LAZYOR: Keycode = Keycode::KeypadLazyOr;
    pub const KC_KP_COLON: Keycode = Keycode::KeypadColon;
    pub const KC_KP_HASH: Keycode = Keycode::KeypadHash;
    pub const KC_KP_SPACE: Keycode = Keycode::KeypadSpace;
    pub const KC_ATMARK: Keycode = Keycode::KeypadAt;
    pub const KC_KP_EXCLAMATION: Keycode = Keycode::KeypadExclamation;
    pub const KC_KP_MEM_STORE: Keycode = Keycode::KeypadMemStore;
    pub const KC_KP_MEM_RECALL: Keycode = Keycode::KeypadMemRecall;
    pub const KC_KP_MEM_CLEAR: Keycode = Keycode::KeypadMemClear;
    pub const KC_KP_MEM_ADD: Keycode = Keycode::KeypadMemAdd;
    pub const KC_KP_MEM_SUB: Keycode = Keycode::KeypadMemSub;
    pub const KC_KP_MEM_MUL: Keycode = Keycode::KeypadMemMul;
    pub const KC_KP_MEM_DIV: Keycode = Keycode::KeypadMemDiv;
    pub const KC_KP_PLUS_MINUS: Keycode = Keycode::KeypadPlusMinus;
    pub const KC_KP_CLEAR: Keycode = Keycode::KeypadClear;
    pub const KC_KP_CLEAR_ENTRY: Keycode = Keycode::KeypadClearEntry;
    pub const KC_KP_BINARY: Keycode = Keycode::KeypadBinary;
    pub const KC_KP_OCTAL: Keycode = Keycode::KeypadOctal;
    pub const KC_KP_DECIMAL: Keycode = Keycode::KeypadDecimal;
    pub const KC_KP_HEXADECIMAL: Keycode = Keycode::KeypadHexadecimal;
    pub const KC_LCTL: Keycode = Keycode::LeftControl;
    pub const KC_LSFT: Keycode = Keycode::LeftShift;
    pub const KC_LALT: Keycode = Keycode::LeftAlt;
    pub const KC_LGUI: Keycode = Keycode::LeftGui;
    pub const KC_RCTL: Keycode = Keycode::RightControl;
    pub const KC_RSFT: Keycode = Keycode::RightShift;
    pub const KC_RALT: Keycode = Keycode::RightAlt;
    pub const KC_RGUI: Keycode = Keycode::RightGui;
}
