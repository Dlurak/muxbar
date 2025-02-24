# Muxbar

Tmux status line configured in Rust.

## Features

- Fully configured in Rust
  - Type-save configuration
  - Can be programmed (e.g. dynamically rendered modules)
- Supports formatting
- Cached modules
    - Each module specifies when it needs to recompute and also how to update it self.
    - Once a module needs to recompute that specific module is updated
    - All other modules are cached

## Installation

1. Clone this repository

   ```bash
   git clone git@github.com:Dlurak/muxbar.git
   ```

2. Install Muxbar

   ```bash
   cargo install --path .
   ```

3. Apply Muxbar in your `.tmux.conf`

   ```text
   set -g status-right '#(muxbar)'
   ```

## Configuration

The configuration is written in Rust and located in `./src/config.rs`

## Examples

![Default](./assets/default.png)
