use crate::colors;
use crate::icons;
use crate::modules::Module;

#[derive(Clone, Copy)]
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

    pub fn display(&self) -> Result<String, ()> {
        let content = self.module.display()?;

        if let Some(icon) = self.icon {
            Ok(format!(
                "{}{} {}{}",
                self.style.display(),
                icon,
                content,
                colors::Style::default().display()
            ))
        } else {
            Ok(format!(
                "{}{}{}",
                self.style.display(),
                content,
                colors::Style::default().display()
            ))
        }
    }
}
