use druid::{AppLauncher, WindowDesc};
use im::Vector;

use crate::{data::TodoState, saver::read_stored, ui::ui_builder};

mod data;
mod saver;
mod ui;

const TITLE: &str = "Todo App";

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(TITLE)
        .window_size((300.0, 400.0));

    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(default_state)
        .expect("Failed to launch application");
}
