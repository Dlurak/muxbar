use modules::modules::{Battery, Cpu, DateTime, Memory, TmuxContent, Warning};
use modules::{modules::ToModule, Color, Icon, Module, Style};

pub fn get_modules() -> Vec<Module<Box<dyn ToModule>>> {
    [
        Some(
            Module::from(DateTime::date())
                .set_icon(Some(Icon::Calendar))
                .set_style(Style::new_with_fg(Color::Yellow)),
        ),
        Some(DateTime::new("%H:%M:%S").into()),
        Some(Cpu::new(2, 2).into()),
        Some(Memory::new(2, 2).into()),
        Battery::try_new(2).ok().flatten().map(|x| x.into()),
        Some(TmuxContent::SessionName.into()),
        Warning::new_battery(20, 20.0)
            .ok()
            .flatten()
            .map(|x| x.into()),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub const PRE_MODULES: &str = "";
pub const BETWEEN_MODULES: &str = " ";
pub const POST_MODULES: &str = " ";
