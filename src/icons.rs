use crossterm::style::Color;

#[derive(Copy, Clone, PartialEq)]
enum IconStyle {
    // Default
    NerdFont,
    // For terminals without nerdfont
    Compatability,
}

static mut STYLE: IconStyle = IconStyle::NerdFont;

pub fn next_style() {
    unsafe {
        STYLE = if STYLE == IconStyle::NerdFont {IconStyle::Compatability} else {IconStyle::NerdFont};
    }
}

pub fn hp() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            IconStyle::NerdFont => (Color::Red, ""),
            _ => (Color::Red, "HP"),
        }
    }
}

pub fn dmg() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            IconStyle::NerdFont => (Color::Blue, "理"),
            _ => (Color::Blue, "DM"),
        }
    }
}

pub fn poison() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            IconStyle::NerdFont => (Color::Green, ""),
            _ => (Color::Green, "PS"),
        }
    }
}

pub fn curse() -> (Color, &'static str) {
    unsafe {
        match STYLE {
            IconStyle::NerdFont => (Color::White, "ﮊ"),
            _ => (Color::White, "CS"),
        }
    }
}
