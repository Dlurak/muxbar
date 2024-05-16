use crate::colors;
use crate::icons;
use crate::modules::Module;

pub struct StyledModule {
    module: Module,
    icon: Option<icons::Icon>,
    style: colors::Style,
}

impl StyledModule {
    pub fn new(module: Module, icon: Option<icons::Icon>, style: colors::Style) -> Self {
        Self {
            module,
            icon,
            style,
        }
    }

    pub fn display(self) -> String {
        if let Some(icon) = self.icon {
            format!(
                "{}{} {}{}",
                self.style.display(),
                icon,
                self.module.display(),
                colors::Style::default().display()
            )
        } else {
            format!(
                "{}{}{}",
                self.style.display(),
                self.module.display(),
                colors::Style::default().display()
            )
        }
    }
}
