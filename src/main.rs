mod colors;
mod config;
mod icons;
mod modules;
mod utils;

use rayon::prelude::*;

fn main() {
    let modules = config::get_modules();

    let parts: Vec<_> = modules
        .par_iter()
        .flat_map(|styled_mod| styled_mod.display())
        .collect();
    let content = parts.join(config::between_modules());

    println!(
        "{}{}{}",
        config::pre_modules(),
        content,
        config::post_modules(),
    );
}
