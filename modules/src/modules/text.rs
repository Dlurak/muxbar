use crate::{Color, Icon, Style};

use super::ToModule;
use std::{fmt, num::NonZero, time::Duration};

pub struct Text {
    text: String,
    width: NonZero<usize>,
    position: usize,
    filler: char,
}

impl Text {
    pub fn new<T>(text: T, padding: usize, filler: char) -> Option<Self>
    where
        T: Into<String>,
    {
        let text = text.into();
        let width = NonZero::new(text.len() + padding)?;

        Some(Self {
            text,
            width,
            filler,
            position: 0,
        })
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text_len = self.text.len();
        let available_length = self.width.get() - self.position;
        let non_wrapped_len = text_len.min(available_length);

        let (non_wrapped, wrapped) = self.text.split_at(non_wrapped_len);

        let filler = self.filler.to_string();

        let pre_len = if wrapped.is_empty() { self.position } else { 0 };
        let between_len = if wrapped.is_empty() {
            0
        } else {
            self.width.get() - text_len
        };
        let post_len = if wrapped.is_empty() {
            self.width.get().saturating_sub(text_len + self.position)
        } else {
            0
        };

        let pre_filler = filler.repeat(pre_len);
        let between_filler = filler.repeat(between_len);
        let post_filler = filler.repeat(post_len);

        write!(
            f,
            "{pre_filler}{wrapped}{between_filler}{non_wrapped}{post_filler}",
        )
    }
}

impl ToModule for Text {
    fn icon(&self) -> Option<Icon> {
        None
    }
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Blue)
    }
    fn next_render_time(&self) -> Option<Duration> {
        if self.width.get() == self.text.len() {
            None
        } else {
            Some(Duration::from_secs(1))
        }
    }
    fn update(&mut self) {
        self.position += 1;
        if self.position >= self.width.get() {
            self.position = 0
        }
    }
}
