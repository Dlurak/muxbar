mod colors;
mod config;
mod icons;
mod modules;
mod utils;

use std::sync::Arc;
use std::thread;

fn main() {
    let modules = config::get_modules();

    let handles = modules.into_iter().map(|styled_mod| {
        let styled_mod = Arc::new(styled_mod);
        let thread_styled_mod = Arc::clone(&styled_mod);

        thread::spawn(move || thread_styled_mod.display().ok())
    });

    let strings: Vec<_> = handles.filter_map(|t| t.join().ok().flatten()).collect();

    println!(
        "{}{}{}",
        config::pre_modules(),
        strings.join(config::between_modules()),
        config::post_modules(),
    );
}
