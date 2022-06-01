#![allow(non_snake_case)]

//! Aliases for keycodes based on the names used in QMK/TMK.

use super::{HidKeycode, Keycode, LayerAction, LayerKeycode, SystemKeycode};

pub const fn MO(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Momentary, layer))
}

pub const fn OSL(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Oneshot, layer))
}

pub const fn TG(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Toggle, layer))
}

pub const fn TO(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::To, layer))
}

pub const KC_NO: Keycode = Keycode::System(SystemKeycode::None);
pub const KC_TRANSPARENT: Keycode = Keycode::System(SystemKeycode::Transparent);
pub const RESET: Keycode = Keycode::System(SystemKeycode::Reset);

pub const XXXXXXX: Keycode = KC_NO;
pub const _______: Keycode = KC_TRNS;

// Short names for ease of definition of keymap
pub const KC_LCTL: Keycode = KC_LCTRL;
pub const KC_RCTL: Keycode = KC_RCTRL;
pub const KC_LSFT: Keycode = KC_LSHIFT;
pub const KC_RSFT: Keycode = KC_RSHIFT;
pub const KC_ESC: Keycode = KC_ESCAPE;
pub const KC_BSPC: Keycode = KC_BSPACE;
pub const KC_ENT: Keycode = KC_ENTER;
pub const KC_DEL: Keycode = KC_DELETE;
pub const KC_INS: Keycode = KC_INSERT;
pub const KC_CAPS: Keycode = KC_CAPSLOCK;
pub const KC_CLCK: Keycode = KC_CAPSLOCK;
pub const KC_RGHT: Keycode = KC_RIGHT;
pub const KC_PGDN: Keycode = KC_PGDOWN;
pub const KC_PSCR: Keycode = KC_PSCREEN;
pub const KC_SLCK: Keycode = KC_SCROLLLOCK;
pub const KC_PAUS: Keycode = KC_PAUSE;
pub const KC_BRK: Keycode = KC_PAUSE;
pub const KC_NLCK: Keycode = KC_NUMLOCK;
pub const KC_SPC: Keycode = KC_SPACE;
pub const KC_MINS: Keycode = KC_MINUS;
pub const KC_EQL: Keycode = KC_EQUAL;
pub const KC_GRV: Keycode = KC_GRAVE;
pub const KC_RBRC: Keycode = KC_RBRACKET;
pub const KC_LBRC: Keycode = KC_LBRACKET;
pub const KC_COMM: Keycode = KC_COMMA;
pub const KC_BSLS: Keycode = KC_BSLASH;
pub const KC_SLSH: Keycode = KC_SLASH;
pub const KC_SCLN: Keycode = KC_SCOLON;
pub const KC_QUOT: Keycode = KC_QUOTE;
pub const KC_APP: Keycode = KC_APPLICATION;
pub const KC_NUHS: Keycode = KC_NONUS_HASH;
pub const KC_NUBS: Keycode = KC_NONUS_BSLASH;
pub const KC_LCAP: Keycode = KC_LOCKING_CAPS;
pub const KC_LNUM: Keycode = KC_LOCKING_NUM;
pub const KC_LSCR: Keycode = KC_LOCKING_SCROLL;
pub const KC_ERAS: Keycode = KC_ALT_ERASE;
pub const KC_CLR: Keycode = KC_CLEAR;
/* Japanese specific */
pub const KC_ZKHK: Keycode = KC_GRAVE;
pub const KC_RO: Keycode = KC_INT1;
pub const KC_KANA: Keycode = KC_INT2;
pub const KC_JYEN: Keycode = KC_INT3;
pub const KC_JPY: Keycode = KC_INT3;
pub const KC_HENK: Keycode = KC_INT4;
pub const KC_MHEN: Keycode = KC_INT5;
/* Korean specific */
pub const KC_HAEN: Keycode = KC_LANG1;
pub const KC_HANJ: Keycode = KC_LANG2;
/* Keypad */
pub const KC_P1: Keycode = KC_KP_1;
pub const KC_P2: Keycode = KC_KP_2;
pub const KC_P3: Keycode = KC_KP_3;
pub const KC_P4: Keycode = KC_KP_4;
pub const KC_P5: Keycode = KC_KP_5;
pub const KC_P6: Keycode = KC_KP_6;
pub const KC_P7: Keycode = KC_KP_7;
pub const KC_P8: Keycode = KC_KP_8;
pub const KC_P9: Keycode = KC_KP_9;
pub const KC_P0: Keycode = KC_KP_0;
pub const KC_P00: Keycode = KC_KP_00;
pub const KC_P000: Keycode = KC_KP_000;
pub const KC_PDOT: Keycode = KC_KP_DOT;
pub const KC_PCMM: Keycode = KC_KP_COMMA;
pub const KC_PSLS: Keycode = KC_KP_SLASH;
pub const KC_PAST: Keycode = KC_KP_ASTERISK;
pub const KC_PMNS: Keycode = KC_KP_MINUS;
pub const KC_PPLS: Keycode = KC_KP_PLUS;
pub const KC_PEQL: Keycode = KC_KP_EQUAL;
pub const KC_PENT: Keycode = KC_KP_ENTER;
/* Unix function key */
pub const KC_EXEC: Keycode = KC_EXECUTE;
pub const KC_SLCT: Keycode = KC_SELECT;
pub const KC_AGIN: Keycode = KC_AGAIN;
pub const KC_PSTE: Keycode = KC_PASTE;
/*TODO
    /* Mousekey */
    pub const KC_MS_U: Keycode = KC_MS_UP;
    pub const KC_MS_D: Keycode = KC_MS_DOWN;
    pub const KC_MS_L: Keycode = KC_MS_LEFT;
    pub const KC_MS_R: Keycode = KC_MS_RIGHT;
    pub const KC_BTN1: Keycode = KC_MS_BTN1;
    pub const KC_BTN2: Keycode = KC_MS_BTN2;
    pub const KC_BTN3: Keycode = KC_MS_BTN3;
    pub const KC_BTN4: Keycode = KC_MS_BTN4;
    pub const KC_BTN5: Keycode = KC_MS_BTN5;
    pub const KC_WH_U: Keycode = KC_MS_WH_UP;
    pub const KC_WH_D: Keycode = KC_MS_WH_DOWN;
    pub const KC_WH_L: Keycode = KC_MS_WH_LEFT;
    pub const KC_WH_R: Keycode = KC_MS_WH_RIGHT;
    pub const KC_ACL0: Keycode = KC_MS_ACCEL0;
    pub const KC_ACL1: Keycode = KC_MS_ACCEL1;
    pub const KC_ACL2: Keycode = KC_MS_ACCEL2;
    /* Sytem Control */
    pub const KC_PWR : Keycode = KC_SYSTEM_POWER;
    pub const KC_SLEP: Keycode = KC_SYSTEM_SLEEP;
    pub const KC_WAKE: Keycode = KC_SYSTEM_WAKE;
    /* Consumer Page */
    pub const KC_MUTE: Keycode = KC_AUDIO_MUTE;
    pub const KC_VOLU: Keycode = KC_AUDIO_VOL_UP;
    pub const KC_VOLD: Keycode = KC_AUDIO_VOL_DOWN;
    pub const KC_MNXT: Keycode = KC_MEDIA_NEXT_TRACK;
    pub const KC_MPRV: Keycode = KC_MEDIA_PREV_TRACK;
    pub const KC_MFFD: Keycode = KC_MEDIA_FAST_FORWARD;
    pub const KC_MRWD: Keycode = KC_MEDIA_REWIND;
    pub const KC_MSTP: Keycode = KC_MEDIA_STOP;
    pub const KC_MPLY: Keycode = KC_MEDIA_PLAY_PAUSE;
    pub const KC_EJCT: Keycode = KC_MEDIA_EJECT;
    pub const KC_MSEL: Keycode = KC_MEDIA_SELECT;
    pub const KC_MAIL: Keycode = KC_MAIL;
    pub const KC_CALC: Keycode = KC_CALCULATOR;
    pub const KC_MYCM: Keycode = KC_MY_COMPUTER;
    pub const KC_WSCH: Keycode = KC_WWW_SEARCH;
    pub const KC_WHOM: Keycode = KC_WWW_HOME;
    pub const KC_WBAK: Keycode = KC_WWW_BACK;
    pub const KC_WFWD: Keycode = KC_WWW_FORWARD;
    pub const KC_WSTP: Keycode = KC_WWW_STOP;
    pub const KC_WREF: Keycode = KC_WWW_REFRESH;
    pub const KC_WFAV: Keycode = KC_WWW_FAVORITES;
    pub const KC_BRTI: Keycode = KC_BRIGHTNESS_INC;
    pub const KC_BRTD: Keycode = KC_BRIGHTNESS_DEC;
    /* Jump to bootloader */
    pub const KC_BTLD: Keycode = KC_BOOTLOADER;
*/
/* Transparent */
pub const KC_TRNS: Keycode = KC_TRANSPARENT;

// Original names from `enum hid_keyboard_keypad_usage`
pub const KC_ROLL_OVER: Keycode = Keycode::Hid(HidKeycode::ErrorRollOver);
pub const KC_POST_FAIL: Keycode = Keycode::Hid(HidKeycode::PostFail);
pub const KC_UNDEFINED: Keycode = Keycode::Hid(HidKeycode::ErrorUndefined);
pub const KC_A: Keycode = Keycode::Hid(HidKeycode::A);
pub const KC_B: Keycode = Keycode::Hid(HidKeycode::B);
pub const KC_C: Keycode = Keycode::Hid(HidKeycode::C);
pub const KC_D: Keycode = Keycode::Hid(HidKeycode::D);
pub const KC_E: Keycode = Keycode::Hid(HidKeycode::E);
pub const KC_F: Keycode = Keycode::Hid(HidKeycode::F);
pub const KC_G: Keycode = Keycode::Hid(HidKeycode::G);
pub const KC_H: Keycode = Keycode::Hid(HidKeycode::H);
pub const KC_I: Keycode = Keycode::Hid(HidKeycode::I);
pub const KC_J: Keycode = Keycode::Hid(HidKeycode::J);
pub const KC_K: Keycode = Keycode::Hid(HidKeycode::K);
pub const KC_L: Keycode = Keycode::Hid(HidKeycode::L);
pub const KC_M: Keycode = Keycode::Hid(HidKeycode::M);
pub const KC_N: Keycode = Keycode::Hid(HidKeycode::N);
pub const KC_O: Keycode = Keycode::Hid(HidKeycode::O);
pub const KC_P: Keycode = Keycode::Hid(HidKeycode::P);
pub const KC_Q: Keycode = Keycode::Hid(HidKeycode::Q);
pub const KC_R: Keycode = Keycode::Hid(HidKeycode::R);
pub const KC_S: Keycode = Keycode::Hid(HidKeycode::S);
pub const KC_T: Keycode = Keycode::Hid(HidKeycode::T);
pub const KC_U: Keycode = Keycode::Hid(HidKeycode::U);
pub const KC_V: Keycode = Keycode::Hid(HidKeycode::V);
pub const KC_W: Keycode = Keycode::Hid(HidKeycode::W);
pub const KC_X: Keycode = Keycode::Hid(HidKeycode::X);
pub const KC_Y: Keycode = Keycode::Hid(HidKeycode::Y);
pub const KC_Z: Keycode = Keycode::Hid(HidKeycode::Z);
pub const KC_1: Keycode = Keycode::Hid(HidKeycode::Num1);
pub const KC_2: Keycode = Keycode::Hid(HidKeycode::Num2);
pub const KC_3: Keycode = Keycode::Hid(HidKeycode::Num3);
pub const KC_4: Keycode = Keycode::Hid(HidKeycode::Num4);
pub const KC_5: Keycode = Keycode::Hid(HidKeycode::Num5);
pub const KC_6: Keycode = Keycode::Hid(HidKeycode::Num6);
pub const KC_7: Keycode = Keycode::Hid(HidKeycode::Num7);
pub const KC_8: Keycode = Keycode::Hid(HidKeycode::Num8);
pub const KC_9: Keycode = Keycode::Hid(HidKeycode::Num9);
pub const KC_0: Keycode = Keycode::Hid(HidKeycode::Num0);
pub const KC_ENTER: Keycode = Keycode::Hid(HidKeycode::Enter);
pub const KC_ESCAPE: Keycode = Keycode::Hid(HidKeycode::Escape);
pub const KC_BSPACE: Keycode = Keycode::Hid(HidKeycode::Backspace);
pub const KC_TAB: Keycode = Keycode::Hid(HidKeycode::Tab);
pub const KC_SPACE: Keycode = Keycode::Hid(HidKeycode::Space);
pub const KC_MINUS: Keycode = Keycode::Hid(HidKeycode::Minus);
pub const KC_EQUAL: Keycode = Keycode::Hid(HidKeycode::Equal);
pub const KC_LBRACKET: Keycode = Keycode::Hid(HidKeycode::LeftBracket);
pub const KC_RBRACKET: Keycode = Keycode::Hid(HidKeycode::RightBracket);
pub const KC_BSLASH: Keycode = Keycode::Hid(HidKeycode::Backslash);
pub const KC_NONUS_HASH: Keycode = Keycode::Hid(HidKeycode::NonUsHash);
pub const KC_SCOLON: Keycode = Keycode::Hid(HidKeycode::Semicolon);
pub const KC_QUOTE: Keycode = Keycode::Hid(HidKeycode::Quote);
pub const KC_GRAVE: Keycode = Keycode::Hid(HidKeycode::Grave);
pub const KC_COMMA: Keycode = Keycode::Hid(HidKeycode::Comma);
pub const KC_DOT: Keycode = Keycode::Hid(HidKeycode::Dot);
pub const KC_SLASH: Keycode = Keycode::Hid(HidKeycode::Slash);
pub const KC_CAPSLOCK: Keycode = Keycode::Hid(HidKeycode::CapsLock);
pub const KC_F1: Keycode = Keycode::Hid(HidKeycode::F1);
pub const KC_F2: Keycode = Keycode::Hid(HidKeycode::F2);
pub const KC_F3: Keycode = Keycode::Hid(HidKeycode::F3);
pub const KC_F4: Keycode = Keycode::Hid(HidKeycode::F4);
pub const KC_F5: Keycode = Keycode::Hid(HidKeycode::F5);
pub const KC_F6: Keycode = Keycode::Hid(HidKeycode::F6);
pub const KC_F7: Keycode = Keycode::Hid(HidKeycode::F7);
pub const KC_F8: Keycode = Keycode::Hid(HidKeycode::F8);
pub const KC_F9: Keycode = Keycode::Hid(HidKeycode::F9);
pub const KC_F10: Keycode = Keycode::Hid(HidKeycode::F10);
pub const KC_F11: Keycode = Keycode::Hid(HidKeycode::F11);
pub const KC_F12: Keycode = Keycode::Hid(HidKeycode::F12);
pub const KC_PSCREEN: Keycode = Keycode::Hid(HidKeycode::PrintScreen);
pub const KC_SCROLLLOCK: Keycode = Keycode::Hid(HidKeycode::ScrollLock);
pub const KC_PAUSE: Keycode = Keycode::Hid(HidKeycode::Pause);
pub const KC_INSERT: Keycode = Keycode::Hid(HidKeycode::Insert);
pub const KC_HOME: Keycode = Keycode::Hid(HidKeycode::Home);
pub const KC_PGUP: Keycode = Keycode::Hid(HidKeycode::PageUp);
pub const KC_DELETE: Keycode = Keycode::Hid(HidKeycode::Delete);
pub const KC_END: Keycode = Keycode::Hid(HidKeycode::End);
pub const KC_PGDOWN: Keycode = Keycode::Hid(HidKeycode::PageDown);
pub const KC_RIGHT: Keycode = Keycode::Hid(HidKeycode::Right);
pub const KC_LEFT: Keycode = Keycode::Hid(HidKeycode::Left);
pub const KC_DOWN: Keycode = Keycode::Hid(HidKeycode::Down);
pub const KC_UP: Keycode = Keycode::Hid(HidKeycode::Up);
pub const KC_NUMLOCK: Keycode = Keycode::Hid(HidKeycode::NumLock);
pub const KC_KP_SLASH: Keycode = Keycode::Hid(HidKeycode::KeypadSlash);
pub const KC_KP_ASTERISK: Keycode = Keycode::Hid(HidKeycode::KeypadAsterisk);
pub const KC_KP_MINUS: Keycode = Keycode::Hid(HidKeycode::KeypadMinus);
pub const KC_KP_PLUS: Keycode = Keycode::Hid(HidKeycode::KeypadPlus);
pub const KC_KP_ENTER: Keycode = Keycode::Hid(HidKeycode::KeypadEnter);
pub const KC_KP_1: Keycode = Keycode::Hid(HidKeycode::Keypad1);
pub const KC_KP_2: Keycode = Keycode::Hid(HidKeycode::Keypad2);
pub const KC_KP_3: Keycode = Keycode::Hid(HidKeycode::Keypad3);
pub const KC_KP_4: Keycode = Keycode::Hid(HidKeycode::Keypad4);
pub const KC_KP_5: Keycode = Keycode::Hid(HidKeycode::Keypad5);
pub const KC_KP_6: Keycode = Keycode::Hid(HidKeycode::Keypad6);
pub const KC_KP_7: Keycode = Keycode::Hid(HidKeycode::Keypad7);
pub const KC_KP_8: Keycode = Keycode::Hid(HidKeycode::Keypad8);
pub const KC_KP_9: Keycode = Keycode::Hid(HidKeycode::Keypad9);
pub const KC_KP_0: Keycode = Keycode::Hid(HidKeycode::Keypad0);
pub const KC_KP_DOT: Keycode = Keycode::Hid(HidKeycode::KeypadDot);
pub const KC_NONUS_BSLASH: Keycode = Keycode::Hid(HidKeycode::NonUsBackslash);
pub const KC_APPLICATION: Keycode = Keycode::Hid(HidKeycode::Application);
pub const KC_POWER: Keycode = Keycode::Hid(HidKeycode::Power);
pub const KC_KP_EQUAL: Keycode = Keycode::Hid(HidKeycode::KeypadEqual);
pub const KC_F13: Keycode = Keycode::Hid(HidKeycode::F13);
pub const KC_F14: Keycode = Keycode::Hid(HidKeycode::F14);
pub const KC_F15: Keycode = Keycode::Hid(HidKeycode::F15);
pub const KC_F16: Keycode = Keycode::Hid(HidKeycode::F16);
pub const KC_F17: Keycode = Keycode::Hid(HidKeycode::F17);
pub const KC_F18: Keycode = Keycode::Hid(HidKeycode::F18);
pub const KC_F19: Keycode = Keycode::Hid(HidKeycode::F19);
pub const KC_F20: Keycode = Keycode::Hid(HidKeycode::F20);
pub const KC_F21: Keycode = Keycode::Hid(HidKeycode::F21);
pub const KC_F22: Keycode = Keycode::Hid(HidKeycode::F22);
pub const KC_F23: Keycode = Keycode::Hid(HidKeycode::F23);
pub const KC_F24: Keycode = Keycode::Hid(HidKeycode::F24);
pub const KC_EXECUTE: Keycode = Keycode::Hid(HidKeycode::Execute);
pub const KC_HELP: Keycode = Keycode::Hid(HidKeycode::Help);
pub const KC_MENU: Keycode = Keycode::Hid(HidKeycode::Menu);
pub const KC_SELECT: Keycode = Keycode::Hid(HidKeycode::Select);
pub const KC_STOP: Keycode = Keycode::Hid(HidKeycode::Stop);
pub const KC_AGAIN: Keycode = Keycode::Hid(HidKeycode::Again);
pub const KC_UNDO: Keycode = Keycode::Hid(HidKeycode::Undo);
pub const KC_CUT: Keycode = Keycode::Hid(HidKeycode::Cut);
pub const KC_COPY: Keycode = Keycode::Hid(HidKeycode::Copy);
pub const KC_PASTE: Keycode = Keycode::Hid(HidKeycode::Paste);
pub const KC_FIND: Keycode = Keycode::Hid(HidKeycode::Find);
pub const KC__MUTE: Keycode = Keycode::Hid(HidKeycode::Mute);
pub const KC__VOLUP: Keycode = Keycode::Hid(HidKeycode::VolumeUp);
pub const KC__VOLDOWN: Keycode = Keycode::Hid(HidKeycode::VolumeDown);
pub const KC_LOCKING_CAPS: Keycode = Keycode::Hid(HidKeycode::LockingCaps);
pub const KC_LOCKING_NUM: Keycode = Keycode::Hid(HidKeycode::LockingNum);
pub const KC_LOCKING_SCROLL: Keycode = Keycode::Hid(HidKeycode::LockingScroll);
pub const KC_KP_COMMA: Keycode = Keycode::Hid(HidKeycode::KeypadComma);
pub const KC_KP_EQUAL_AS400: Keycode = Keycode::Hid(HidKeycode::KeypadEqualAS400);
pub const KC_INT1: Keycode = Keycode::Hid(HidKeycode::Int1);
pub const KC_INT2: Keycode = Keycode::Hid(HidKeycode::Int2);
pub const KC_INT3: Keycode = Keycode::Hid(HidKeycode::Int3);
pub const KC_INT4: Keycode = Keycode::Hid(HidKeycode::Int4);
pub const KC_INT5: Keycode = Keycode::Hid(HidKeycode::Int5);
pub const KC_INT6: Keycode = Keycode::Hid(HidKeycode::Int6);
pub const KC_INT7: Keycode = Keycode::Hid(HidKeycode::Int7);
pub const KC_INT8: Keycode = Keycode::Hid(HidKeycode::Int8);
pub const KC_INT9: Keycode = Keycode::Hid(HidKeycode::Int9);
pub const KC_LANG1: Keycode = Keycode::Hid(HidKeycode::Lang1);
pub const KC_LANG2: Keycode = Keycode::Hid(HidKeycode::Lang2);
pub const KC_LANG3: Keycode = Keycode::Hid(HidKeycode::Lang3);
pub const KC_LANG4: Keycode = Keycode::Hid(HidKeycode::Lang4);
pub const KC_LANG5: Keycode = Keycode::Hid(HidKeycode::Lang5);
pub const KC_LANG6: Keycode = Keycode::Hid(HidKeycode::Lang6);
pub const KC_LANG7: Keycode = Keycode::Hid(HidKeycode::Lang7);
pub const KC_LANG8: Keycode = Keycode::Hid(HidKeycode::Lang8);
pub const KC_LANG9: Keycode = Keycode::Hid(HidKeycode::Lang9);
pub const KC_ALT_ERASE: Keycode = Keycode::Hid(HidKeycode::AltErase);
pub const KC_SYSREQ: Keycode = Keycode::Hid(HidKeycode::SysReq);
pub const KC_CANCEL: Keycode = Keycode::Hid(HidKeycode::Cancel);
pub const KC_CLEAR: Keycode = Keycode::Hid(HidKeycode::Clear);
pub const KC_PRIOR: Keycode = Keycode::Hid(HidKeycode::Prior);
pub const KC_RETURN: Keycode = Keycode::Hid(HidKeycode::Return);
pub const KC_SEPARATOR: Keycode = Keycode::Hid(HidKeycode::Separator);
pub const KC_OUT: Keycode = Keycode::Hid(HidKeycode::Out);
pub const KC_OPER: Keycode = Keycode::Hid(HidKeycode::Oper);
pub const KC_CLEAR_AGAIN: Keycode = Keycode::Hid(HidKeycode::ClearAgain);
pub const KC_CRSEL: Keycode = Keycode::Hid(HidKeycode::CrSel);
pub const KC_EXSEL: Keycode = Keycode::Hid(HidKeycode::ExSel);
pub const KC_KP_00: Keycode = Keycode::Hid(HidKeycode::Keypad00);
pub const KC_KP_000: Keycode = Keycode::Hid(HidKeycode::Keypad000);
pub const KC_THOUSANDS_SEPARATOR: Keycode = Keycode::Hid(HidKeycode::ThousandsSep);
pub const KC_DECIMAL_SEPARATOR: Keycode = Keycode::Hid(HidKeycode::DecimalSep);
pub const KC_CURRENCY_UNIT: Keycode = Keycode::Hid(HidKeycode::CurrencyUnit);
pub const KC_CURRENCY_SUB_UNIT: Keycode = Keycode::Hid(HidKeycode::CurrencySubUnit);
pub const KC_KP_LPAREN: Keycode = Keycode::Hid(HidKeycode::KeypadLeftParen);
pub const KC_KP_RPAREN: Keycode = Keycode::Hid(HidKeycode::KeypadRightParen);
pub const KC_KP_LCBRACKET: Keycode = Keycode::Hid(HidKeycode::KeypadLeftCBracket);
pub const KC_KP_RCBRACKET: Keycode = Keycode::Hid(HidKeycode::KeypadRightCBracket);
pub const KC_KP_TAB: Keycode = Keycode::Hid(HidKeycode::KeypadTab);
pub const KC_KP_BSPACE: Keycode = Keycode::Hid(HidKeycode::KeypadBackspace);
pub const KC_KP_A: Keycode = Keycode::Hid(HidKeycode::KeypadA);
pub const KC_KP_B: Keycode = Keycode::Hid(HidKeycode::KeypadB);
pub const KC_KP_C: Keycode = Keycode::Hid(HidKeycode::KeypadC);
pub const KC_KP_D: Keycode = Keycode::Hid(HidKeycode::KeypadD);
pub const KC_KP_E: Keycode = Keycode::Hid(HidKeycode::KeypadE);
pub const KC_KP_F: Keycode = Keycode::Hid(HidKeycode::KeypadF);
pub const KC_KP_XOR: Keycode = Keycode::Hid(HidKeycode::KeypadXor);
pub const KC_KP_HAT: Keycode = Keycode::Hid(HidKeycode::KeypadHat);
pub const KC_KP_PERC: Keycode = Keycode::Hid(HidKeycode::KeypadPercent);
pub const KC_KP_LT: Keycode = Keycode::Hid(HidKeycode::KeypadLess);
pub const KC_KP_GT: Keycode = Keycode::Hid(HidKeycode::KeypadGreater);
pub const KC_KP_AND: Keycode = Keycode::Hid(HidKeycode::KeypadAnd);
pub const KC_KP_LAZYAND: Keycode = Keycode::Hid(HidKeycode::KeypadLazyAnd);
pub const KC_KP_OR: Keycode = Keycode::Hid(HidKeycode::KeypadOr);
pub const KC_KP_LAZYOR: Keycode = Keycode::Hid(HidKeycode::KeypadLazyOr);
pub const KC_KP_COLON: Keycode = Keycode::Hid(HidKeycode::KeypadColon);
pub const KC_KP_HASH: Keycode = Keycode::Hid(HidKeycode::KeypadHash);
pub const KC_KP_SPACE: Keycode = Keycode::Hid(HidKeycode::KeypadSpace);
pub const KC_ATMARK: Keycode = Keycode::Hid(HidKeycode::KeypadAt);
pub const KC_KP_EXCLAMATION: Keycode = Keycode::Hid(HidKeycode::KeypadExclamation);
pub const KC_KP_MEM_STORE: Keycode = Keycode::Hid(HidKeycode::KeypadMemStore);
pub const KC_KP_MEM_RECALL: Keycode = Keycode::Hid(HidKeycode::KeypadMemRecall);
pub const KC_KP_MEM_CLEAR: Keycode = Keycode::Hid(HidKeycode::KeypadMemClear);
pub const KC_KP_MEM_ADD: Keycode = Keycode::Hid(HidKeycode::KeypadMemAdd);
pub const KC_KP_MEM_SUB: Keycode = Keycode::Hid(HidKeycode::KeypadMemSub);
pub const KC_KP_MEM_MUL: Keycode = Keycode::Hid(HidKeycode::KeypadMemMul);
pub const KC_KP_MEM_DIV: Keycode = Keycode::Hid(HidKeycode::KeypadMemDiv);
pub const KC_KP_PLUS_MINUS: Keycode = Keycode::Hid(HidKeycode::KeypadPlusMinus);
pub const KC_KP_CLEAR: Keycode = Keycode::Hid(HidKeycode::KeypadClear);
pub const KC_KP_CLEAR_ENTRY: Keycode = Keycode::Hid(HidKeycode::KeypadClearEntry);
pub const KC_KP_BINARY: Keycode = Keycode::Hid(HidKeycode::KeypadBinary);
pub const KC_KP_OCTAL: Keycode = Keycode::Hid(HidKeycode::KeypadOctal);
pub const KC_KP_DECIMAL: Keycode = Keycode::Hid(HidKeycode::KeypadDecimal);
pub const KC_KP_HEXADECIMAL: Keycode = Keycode::Hid(HidKeycode::KeypadHexadecimal);
pub const KC_LCTRL: Keycode = Keycode::Hid(HidKeycode::LeftControl);
pub const KC_LSHIFT: Keycode = Keycode::Hid(HidKeycode::LeftShift);
pub const KC_LALT: Keycode = Keycode::Hid(HidKeycode::LeftAlt);
pub const KC_LGUI: Keycode = Keycode::Hid(HidKeycode::LeftGui);
pub const KC_RCTRL: Keycode = Keycode::Hid(HidKeycode::RightControl);
pub const KC_RSHIFT: Keycode = Keycode::Hid(HidKeycode::RightShift);
pub const KC_RALT: Keycode = Keycode::Hid(HidKeycode::RightAlt);
pub const KC_RGUI: Keycode = Keycode::Hid(HidKeycode::RightGui);

// Backlight keycodes https://docs.qmk.fm/#/feature_backlight
pub const BL_DEC: Keycode = Keycode::System(SystemKeycode::BacklightDown);
pub const BL_INC: Keycode = Keycode::System(SystemKeycode::BacklightUp);
pub const BL_STEP: Keycode = Keycode::System(SystemKeycode::BacklightStep);
