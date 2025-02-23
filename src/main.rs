mod config;

use rayon::prelude::*;

fn main() {
    let modules = config::get_modules();

    let parts: Vec<_> = modules
        .par_iter()
        .map(|styled_mod| styled_mod.to_string())
        .collect();
    let content = parts.join(config::BETWEEN_MODULES);

    println!("{}{}{}", config::PRE_MODULES, content, config::POST_MODULES,);
}
