use druid::{
    theme::{BUTTON_BORDER_RADIUS, BUTTON_DARK, BUTTON_LIGHT},
    AppLauncher, Color, WindowDesc,
};
use im::Vector;

use crate::{data::TodoState, saver::read_stored, ui::ui_builder};

mod data;
mod saver;
mod ui;

const TITLE: &str = "Todo App";

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(TITLE)
        .window_size((400.0, 400.0));

    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_BORDER_RADIUS, 0.0);
            env.set(BUTTON_DARK, Color::rgba8(30, 30, 30, 255));
            env.set(BUTTON_LIGHT, Color::rgba8(30, 30, 30, 255));
        })
        .log_to_console()
        .launch(default_state)
        .expect("Failed to launch application");
}
