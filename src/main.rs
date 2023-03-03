use druid::{AppLauncher, WindowDesc};

use crate::ui::ui_builder;

mod data;
mod ui;

const TITLE: &str = "Todo App";

pub fn main() {
    println!("hello");
    println!("{:?}", data::TodoState::default());

    let main_window = WindowDesc::new(ui_builder())
        .title(TITLE)
        .window_size((600.0, 400.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data::TodoState::default())
        .expect("Failed to launch application");
}

// #![windows_subsystem = "windows"]

// use druid::widget::{prelude::*, Button};
// use druid::widget::{Flex, Label, TextBox};
// use druid::{AppLauncher, Color, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

// #[derive(Clone, Data, Lens)]
// struct MainState {
//     name: String,
// }

// pub fn main() {
//     // describe the main window
//     let main_window = WindowDesc::new(build_root_widget())
//         .title("Starting Example")
//         .window_size((600.0, 400.0));

//     // create the initial app state
//     let initial_state: MainState = MainState {
//         name: "World".to_string(),
//     };

//     // start the application. Here we pass in the application state.
//     AppLauncher::with_window(main_window)
//         .log_to_console()
//         .launch(initial_state)
//         .expect("Failed to launch application");
// }

// fn build_root_widget() -> impl Widget<MainState> {
//     let label2_method = |data: &MainState, _env: &Env| {
//         if data.name.is_empty() {
//             "Label 2 Hello anybody!?".to_string()
//         } else {
//             format!("Lalal {}!", data.name)
//         }
//     };

//     let label = Label::new(label2_method).with_text_size(32.0);
//     let label2 = Label::new(label2_method).with_text_size(32.0);

//     // a textbox that modifies `name`.
//     let textbox = TextBox::new()
//         .with_placeholder("Who are we greeting?")
//         .with_text_size(18.0)
//         .fix_width(200.0)
//         .lens(MainState::name);

//     let orange = Color::rgb(252.0, 186.0, 3.0);

//     let btn = Button::new("Wsuup");

//     btn.foreground(orange).set_background(orange);

//     // btn.try_into();

//     // .on_click(|c, data: &mut MainState, _| {
//     //     data.name = "from button".to_string();

//     //     if (c.is_focused()) {}
//     // });

//     return Flex::column()
//         .with_child(label)
//         .with_child(label2)
//         .with_child(btn)
//         .with_spacer(20.0)
//         .with_child(textbox)
//         .align_vertical(UnitPoint::CENTER);
// }
