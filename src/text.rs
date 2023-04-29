use crossterm::{queue, style, style::Color};

#[derive(Copy, Clone)]
pub enum TerminalStyle {
    // For terminals without nerdfont
    Compability,
    NerdFont,
    Total,
}

static mut STYLE: TerminalStyle = TerminalStyle::Compability;

pub fn next_style() {
    unsafe {
        STYLE = std::mem::transmute((STYLE as u8 + 1) % (TerminalStyle::Total as u8));
    }
}


pub fn get_hp_text() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            TerminalStyle::NerdFont => (Color::Red, " "),
            _ => (Color::Red, "HP"),
        }
        }
}

pub fn get_dmg_text() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            TerminalStyle::NerdFont => (Color::Blue, "理  "),
            _ => (Color::Blue, "DMG"),
        }
    }
}

pub fn get_poison_text() -> (Color, &'static str) {

    unsafe {
    match STYLE {
        TerminalStyle::NerdFont => (Color::Green, "  "),
        _ => (Color::Green, "Po "),
    }
    }

}

pub fn get_curse_text() -> (Color, &'static str) {

    unsafe {
    match STYLE {
        TerminalStyle::NerdFont => (Color::White, "ﮊ  "),
        _ => (Color::White, "Cu "),
    }
    }
}