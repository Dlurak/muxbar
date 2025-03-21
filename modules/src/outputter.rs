use crate::{Color, Style};

pub trait Outputter {
    fn prefix(&self, style: Style) -> String;
    fn postfix(&self, _style: Style) -> Option<String> {
        None
    }
}

pub struct TmuxOutputter;

impl TmuxOutputter {
    fn get_tmux_color_code(color: Color, background: bool) -> String {
        let prefix = if background { "bg" } else { "fg" };
        match color {
            Color::White => format!("#[{}=white]", prefix),
            Color::Black => format!("#[{}=black]", prefix),
            Color::Red => format!("#[{}=red]", prefix),
            Color::Green => format!("#[{}=green]", prefix),
            Color::Yellow => format!("#[{}=yellow]", prefix),
            Color::Blue => format!("#[{}=blue]", prefix),
            Color::Cyan => format!("#[{}=cyan]", prefix),
            Color::Magenta => format!("#[{}=magenta]", prefix),
            Color::Reset => format!("#[{}=default]", prefix),
            Color::Any(color) => format!("#[{}={}]", prefix, color),
        }
    }
}

impl Outputter for TmuxOutputter {
    fn prefix(&self, style: Style) -> String {
        let bold_prefix = if style.bold { "#[bold]" } else { "#[nobold]" };
        format!(
            "{}{}{}",
            Self::get_tmux_color_code(style.fg, false),
            Self::get_tmux_color_code(style.bg, true),
            bold_prefix
        )
    }
}

pub struct TtyOutputter;

impl Outputter for TtyOutputter {
    fn prefix(&self, style: Style) -> String {
        let mut ansi_style = ansi_term::Style::new();

        if let Ok(fg_color) = style.fg.try_into() {
            ansi_style = ansi_style.fg(fg_color);
        }

        if let Ok(bg_color) = style.bg.try_into() {
            ansi_style = ansi_style.on(bg_color);
        }

        if style.bold {
            ansi_style = ansi_style.bold();
        }

        ansi_style.prefix().to_string()
    }
}
