use crate::data::{TodoItem, TodoState};
use druid::{
    widget::{Button, Checkbox, Flex, Label, List, TextBox},
    Env, Widget, WidgetExt,
};

pub fn build_header() -> impl Widget<TodoState> {
    let text_input = TextBox::new().lens(TodoState::new_text).expand_width();
    let button = Button::new("Add").on_click(|_ctx, data: &mut TodoState, _env| {
        if data.new_text.trim() != "" {
            let text = data.new_text.clone();
            data.new_text = "".to_string();

            let new_todo = TodoItem {
                text: text.to_string(),
                ..Default::default()
            };

            data.todos.push_front(new_todo);
        }

        // println!("{:?}", data);
    });

    return Flex::row()
        .with_flex_child(text_input, 1.0)
        .with_child(button);
}

pub fn build_todos() -> impl Widget<TodoState> {
    return List::new(|| {
        let checkbox = Checkbox::new("").lens(TodoItem::checked);
        let label = Label::new(|data: &TodoItem, _: &Env| data.text.clone());

        // Checkbox, Text, Button (del)
        return Flex::row().with_child(checkbox).with_child(label);
    })
    .lens(TodoState::todos)
    .scroll();
}

pub fn ui_builder() -> impl Widget<TodoState> {
    return Flex::column()
        .with_child(build_header())
        .with_flex_child(build_todos(), 1.0);
}
