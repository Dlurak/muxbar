
# Modules

Modules are defined in this directory.

## Contributing

To add a custom module, follow these steps:

1. Write a struct that represents your module and implement the `Display` trait
   for it.
2. Use the `Module` struct to apply Icons and Styles to your module.
3. Add your module to the `get_modules` function in `src/config.rs`.

### Example

Here's a basic example of how to create a custom module:

1. Define your module in a new file, e.g., `src/modules/custom.rs`:

    ```rust
    use std::fmt;
    use crate::colors::{Color, Style};
    use crate::icons::Icon;
    use super::Module;

    pub struct CustomModule;

    impl fmt::Display for CustomModule {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Custom Module Content")
        }
    }

    impl CustomModule {
        pub fn new() -> Box<Module<CustomModule>> {
            Box::new(Module::new(
                CustomModule,
                Some(Icon::Custom),
                Style {
                    fg: Color::Blue,
                    bg: Color::Reset,
                    bold: true,
                },
            ))
        }
    }
    ```

2. Add your module to the `get_modules` function in `src/config.rs`:

    ```rust
    use crate::modules::custom::CustomModule;

    pub fn get_modules() -> Vec<Box<dyn Display + Send>> {
        vec![
            // Existing modules...
            CustomModule::new(),
        ]
    }
    ```

3. Update `src/modules/mod.rs` to include your new module:

    ```rust
    pub mod custom;
    ```

If you decide to implement a custom module, I would be glad to see a PR :)
