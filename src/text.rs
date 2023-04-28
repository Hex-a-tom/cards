use crossterm::{queue, style, style::Color};

#[derive(Copy, Clone)]
pub enum TerminalStyle {
    AllText,
    NerdFont,
}

static mut STYLE: TerminalStyle = TerminalStyle::AllText;

fn change_style (t: TerminalStyle) {
    unsafe {
        STYLE = t;
    }
}
pub fn next_style() {
    unsafe {
    match STYLE {
        TerminalStyle::AllText => STYLE = TerminalStyle::NerdFont,
        TerminalStyle::NerdFont => STYLE = TerminalStyle::AllText,
    }
    }
}

pub fn get_poison_text() -> (Color, &'static str) {

    unsafe {
    use TerminalStyle::*;
    match STYLE {
        AllText => (Color::Green, "Po "),
        NerdFont => (Color::Red, "AAAA "),
    }
    }

}