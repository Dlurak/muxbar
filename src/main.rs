mod colors;
mod config;
mod modules;

fn main() {
    let modules = config::get_modules();

    let strings = modules
        .into_iter()
        .map(|styled_mod| styled_mod.display())
        .collect::<Vec<_>>();

    println!(
        "{}{}{}",
        config::pre_modules(),
        strings.join(config::between_modules()),
        config::post_modules(),
    );
}
