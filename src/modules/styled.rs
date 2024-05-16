use crate::colors;
use crate::modules::Module;

pub struct StyledModule {
    module: Module,
    style: colors::Style,
}

impl StyledModule {
    pub fn new(module: Module, style: colors::Style) -> Self {
        Self { module, style }
    }

    pub fn display(self) -> String {
        format!(
            "{}{}{}",
            self.style.display(),
            self.module.display(),
            colors::Style::default().display()
        )
    }
}
