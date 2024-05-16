mod colors;
mod config;
mod icons;
mod modules;

fn main() {
    let modules = config::get_modules();

    let strings = modules
        .into_iter()
        .filter_map(|styled_mod| styled_mod.display().ok())
        .collect::<Vec<_>>();

    println!(
        "{}{}{}",
        config::pre_modules(),
        strings.join(config::between_modules()),
        config::post_modules(),
    );
}
