use crate::{
    data::{TodoItem, TodoState},
    saver::Saver,
};

use druid::{
    widget::{Button, Checkbox, Flex, Label, List, TextBox},
    Env, EventCtx, Insets, Menu, MenuItem, Point, Widget, WidgetExt,
};

pub fn build_header() -> impl Widget<TodoState> {
    let text_input = TextBox::new().lens(TodoState::new_text).expand_width();

    let button = Button::new("Add").on_click(|_ctx, data: &mut TodoState, _env| {
        if data.new_text.trim() != "" {
            let text = data.new_text.clone();
            data.clear_text();

            let new_todo = TodoItem {
                text: text.to_string(),
                ..Default::default()
            };

            data.todos.push_front(new_todo);
        }
    });

    return Flex::row()
        .with_flex_child(text_input, 1.0)
        .with_child(button)
        .padding(Insets::uniform(15.0));
}

pub fn build_todos() -> impl Widget<TodoState> {
    return List::new(|| {
        let checkbox = Checkbox::new("").lens(TodoItem::checked);
        let label = Label::new(|data: &TodoItem, _: &Env| data.text.clone());
        let button =
            Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                let data_clone = data.clone(); // Should be cheap

                let button_menu: Menu<TodoState> = Menu::empty().entry(
                    MenuItem::new("Remove").on_activate(move |_, main_data: &mut TodoState, _| {
                        let index = main_data.todos.index_of(&data_clone).unwrap();

                        main_data.todos.remove(index);
                    }),
                );

                ctx.show_context_menu(button_menu, Point::new(0.0, 0.0));
            });

        // Checkbox, Text, Button (del)
        return Flex::row()
            .with_child(checkbox)
            .with_child(label)
            .with_flex_spacer(0.1)
            .with_child(button)
            .padding(Insets::uniform_xy(15.0, 0.0));
    })
    .lens(TodoState::todos)
    .scroll()
    .vertical();
}

pub fn ui_builder() -> impl Widget<TodoState> {
    return Flex::column()
        .with_child(build_header())
        .with_flex_child(build_todos(), 1.0)
        .with_child(Saver {});
}
