use lazy_static::lazy_static;
use tui::style::{Color, Style};

pub fn style() -> Style {
    Style::default().bg(THEME.background())
}

macro_rules! def_theme_struct_with_defaults {
    ($($name:ident => $color:expr),+) => {
        pub struct Theme {
            $(
            $name: Option<Color>,
            )+
        }

        impl Theme {
            $(
                fn $name(&self) -> Color {
                    self.$name.unwrap_or($color)
                }
            )+
        }

        impl Default for Theme {
            fn default() -> Self {
                Self{
                    $(
                        $name: Some($color),
                    )+
                }
            }
        }
    };
}

def_theme_struct_with_defaults!(
    background => Color::Reset,
    // gray => Color::DarkGray,
    // profit => Color::Green,
    // loss => Color::Red,
    // text_normal => Color::Reset,
    // text_primary => Color::Yellow,
    // text_secondary => Color::Cyan,
    // border_primary => Color::Blue,
    // border_secondary => Color::Reset,
    // border_axis => Color::Blue,
    // highlight_focused => Color::LightBlue,
    highlight_unfocused => Color::DarkGray
);

lazy_static! {
    static ref THEME: Theme = Default::default();
}
