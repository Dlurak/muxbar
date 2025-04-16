mod config;

use itertools::Itertools;
use modules::{
    modules::ToModule,
    outputter::{self, Outputter, TmuxOutputter, TtyOutputter},
    Module, Style,
};
use std::{
    mem, thread,
    time::{Duration, Instant},
};

type Inner = Box<dyn ToModule>;

struct CacheEntry<'a> {
    module: &'a mut Module<Inner>,
    cached_str: String,
    last_render_time: Instant,
    rerender_interval: Option<Duration>,
    include: bool,
}

struct Cache<'a>(Vec<CacheEntry<'a>>);

impl<'a> Cache<'a> {
    fn new(entries: Vec<CacheEntry<'a>>) -> Self {
        Self(entries)
    }

    fn update(self, outputter: &dyn Outputter) -> (Self, bool) {
        let now = Instant::now();
        let mut rerender = false;

        let vec = self
            .0
            .into_iter()
            .map(|mut entry| {
                let needs_rerender = entry
                    .rerender_interval
                    .is_some_and(|i| entry.last_render_time + i <= now);

                if needs_rerender {
                    rerender = true;
                    entry.module.update();
                    entry
                        .module
                        .internal_set_mut_icon(entry.module.content.icon());
                    entry
                        .module
                        .internal_set_mut_style(entry.module.content.style());
                    entry.rerender_interval = entry.module.content.next_render_time();
                    entry.cached_str = entry.module.output(outputter);
                    entry.include = entry.module.content.include();
                }
                entry
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
            .flat_map(|entry| entry.include.then_some(&entry.cached_str))
            .join(config::BETWEEN_MODULES)
    }
}

fn main() {
    let outputter = std::env::args()
        .nth(1)
        .and_then(outputter::new_outputter)
        .unwrap_or(if cfg!(debug_assertions) {
            &TtyOutputter
        } else {
            &TmuxOutputter
        });

    let mods = config::get_modules();
    let cached_modules: Vec<_> = mods
        .iter()
        .map(|module| {
            (
                module.output(outputter),
                module.content.next_render_time(),
                module.content.include(),
            )
        })
        .collect();
    let mut cached_modules: Vec<_> = mods.into_iter().zip(cached_modules).collect();
    let now = Instant::now();
    let mut cache = Cache::new(
        cached_modules
            .iter_mut()
            .map(
                |(module, (cached_str, next_render_time, include))| CacheEntry {
                    module,
                    last_render_time: now,
                    cached_str: mem::take(cached_str),
                    rerender_interval: mem::take(next_render_time),
                    include: mem::take(include),
                },
            )
            .collect(),
    );

    let mut first_render = false;
    loop {
        let now = Instant::now();

        let updated = cache.update(outputter);
        cache = updated.0;

        if updated.1 || !first_render {
            first_render = true;
            println!(
                "{}{}{}{}",
                config::PRE_MODULES,
                String::from(&cache),
                config::POST_MODULES,
                outputter.prefix(Style::default())
            );
        }

        let next_render_times = cache
            .0
            .iter()
            .filter_map(|entry| entry.rerender_interval.map(|d| entry.last_render_time + d));
        let earliest_update = next_render_times.min();
        let next_duration = earliest_update.map(|time| time - now);
        match next_duration {
            Some(duration) => thread::sleep(duration.max(config::FASTEST_INTERVALL)),
            None => loop {
                // stay idle without stopping the program
                thread::park()
            },
        }
    }
}
