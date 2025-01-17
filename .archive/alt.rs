use muxbar::{
    colors::{Color, Style},
    icons::Icon,
    modules::{content::Module, styled::StyledModule},
};

fn main() {
    let clock = StyledModule::new(
        Module::Time("%H:%M:%S".to_string()),
        Some(Icon::Time),
        Style {
            fg: Color::Magenta,
            bg: Color::Reset,
            bold: false,
        },
    );
    println!("{}", clock);
}
