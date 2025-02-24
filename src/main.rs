mod config;

use itertools::Itertools;
use modules::{modules::ToModule, Module, Style};
use std::{thread, time::Instant};

type Inner = Box<dyn ToModule>;
type CacheEntry<'a> = (&'a mut Module<Inner>, (String, Option<Instant>, bool));
struct Cache<'a>(Vec<CacheEntry<'a>>);

impl<'a> Cache<'a> {
    fn new(inner: Vec<CacheEntry<'a>>) -> Self {
        Cache(inner)
    }

    fn update(self) -> (Self, bool) {
        let now = Instant::now();
        let mut rerender = false;

        let vec = self
            .0
            .into_iter()
            .map(|(module, (cached_str, next_render_time, include))| {
                let needs_rerender = next_render_time.is_some_and(|t| now >= t);

                if needs_rerender {
                    rerender = true;
                    module.update();
                    module.internal_set_mut_icon(module.content.icon());
                    module.internal_set_mut_style(module.content.style());
                    let new_next_render_time = module.content.next_render_time();
                    let new_str = module.to_string();
                    let new_include = module.content.include();
                    (module, (new_str, new_next_render_time, new_include))
                } else {
                    (module, (cached_str, next_render_time, include))
                }
            })
            .collect();

        (Cache(vec), rerender)
    }
}

impl From<&Cache<'_>> for String {
    fn from(value: &Cache) -> Self {
        value
            .0
            .iter()
            .flat_map(|(_module, (content, _, include))| include.then_some(content))
            .join(config::BETWEEN_MODULES)
    }
}

fn main() {
    let cached_modules: Vec<_> = config::get_modules()
        .into_iter()
        .map(|module| {
            (
                module.to_string(),
                module.content.next_render_time(),
                module.content.include(),
            )
        })
        .collect();
    let mut cached_modules: Vec<_> = config::get_modules()
        .into_iter()
        .zip(cached_modules)
        .collect();
    let mut cache = Cache::new(
        cached_modules
            .iter_mut()
            .map(|(module, (cached_str, next_render_time, include))| {
                let cached_str = std::mem::take(cached_str);
                let next_render_time = std::mem::take(next_render_time);
                let include = std::mem::take(include);
                (module, (cached_str, next_render_time, include))
            })
            .collect(),
    );

    let mut first_render = false;
    loop {
        let now = Instant::now();

        let updated = cache.update();
        cache = updated.0;

        if updated.1 || !first_render {
            first_render = true;
            println!(
                "{}{}{}{}",
                config::PRE_MODULES,
                String::from(&cache),
                config::POST_MODULES,
                Style::default().display()
            );
        }

        let earliest_update = cache
            .0
            .iter()
            .filter_map(|(_mod, (_str, duration, _include))| *duration)
            .min();
        let next_duration = earliest_update.map(|time| time - now);
        match next_duration {
            Some(duration) => thread::sleep(duration),
            None => loop {
                // stay idle without stopping the programm
                thread::park()
            },
        }
    }
}
