use iced::{
    widget::{
        button, checkbox, column, container, horizontal_rule, horizontal_space, row, scrollable, 
        text, text_input, vertical_space
    },
    Alignment, Element, Length,
};

use crate::todo::{Filter, Message, Todo, TodoState};
use super::style::{ButtonStyle, ContainerStyle, CheckboxStyle, TextInputStyle};

pub fn view_app(state: &TodoState) -> Element<Message> {
    let title = text("Todo App")
        .size(28)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    let add_todo_input = view_add_todo_input(state);
    let search_and_filter = view_search_and_filter(state);
    let todo_list = view_todo_list(state);

    container(
        column![
            vertical_space(20),
            title,
            vertical_space(20),
            add_todo_input,
            vertical_space(20),
            search_and_filter,
            vertical_space(10),
            horizontal_rule(1),
            vertical_space(10),
            todo_list,
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center)
    )
    .style(iced::theme::Container::Custom(Box::new(ContainerStyle::MainContainer)))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20)
    .into()
}

fn view_add_todo_input(state: &TodoState) -> Element<Message> {
    let default_string = String::new();
    let (id, input_value) = state.editing().unwrap_or((0, &default_string));
    
    let input = text_input("Add a new todo...", input_value)
        .on_input(Message::InputChanged)
        .on_submit(Message::AddTodo)
        .padding(10)
        .width(Length::Fill)
        .style(iced::theme::TextInput::Custom(
            Box::new(if id == 0 { TextInputStyle::Focused } else { TextInputStyle::Default })
        ));
    
    let add_button = button(text("Add").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(Message::AddTodo)
        .padding(10)
        .width(Length::Fixed(80.0))
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Add)));

    row![input, horizontal_space(10), add_button]
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn view_search_and_filter(state: &TodoState) -> Element<Message> {
    let search_input = text_input("Search todos...", &state.search_query())
        .on_input(Message::SearchQueryChanged)
        .padding(10)
        .width(Length::Fill)
        .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle::Default)));

    let filter_all = button(text("All"))
        .on_press(Message::FilterChanged(Filter::All))
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(
            if *state.filter() == Filter::All {
                ButtonStyle::FilterActive
            } else {
                ButtonStyle::Filter
            }
        )));

    let filter_active = button(text("Active"))
        .on_press(Message::FilterChanged(Filter::Active))
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(
            if *state.filter() == Filter::Active {
                ButtonStyle::FilterActive
            } else {
                ButtonStyle::Filter
            }
        )));

    let filter_completed = button(text("Completed"))
        .on_press(Message::FilterChanged(Filter::Completed))
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(
            if *state.filter() == Filter::Completed {
                ButtonStyle::FilterActive
            } else {
                ButtonStyle::Filter
            }
        )));

    row![
        search_input,
        horizontal_space(10),
        filter_all,
        filter_active,
        filter_completed
    ]
    .spacing(5)
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn view_todo_list(state: &TodoState) -> Element<Message> {
    let todos = state.filtered_todos();
    
    let items: Vec<Element<_>> = todos
        .iter()
        .map(|todo| view_todo_item(todo))
        .collect();

    if items.is_empty() {
        container(
            text("No todos yet! Add one above.")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .width(Length::Fill)
        .center_x()
        .into()
    } else {
        scrollable(
            column(items)
                .spacing(10)
                .width(Length::Fill)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

fn view_todo_item(todo: &Todo) -> Element<Message> {
    let checkbox = checkbox(
        "",
        todo.completed,
        move |_| Message::ToggleTodo(todo.id)
    )
    .style(iced::theme::Checkbox::Custom(Box::new(CheckboxStyle::Todo)));

    let title = text(&todo.title)
        .width(Length::Fill)
        .size(18);
    
    let title = if todo.completed {
        title.style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
    } else {
        title
    };

    let description = if !todo.description.is_empty() {
        text(&todo.description)
            .size(14)
            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
    } else {
        text("")
    };

    let delete_button = button(text("Delete").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(Message::DeleteTodo(todo.id))
        .padding(5)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Delete)));
    
    let edit_button = button(text("Edit").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(Message::EditingTodo(todo.id, todo.title.clone()))
        .padding(5)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Filter)));

    let row = row![
        checkbox,
        column![title, description].spacing(5).width(Length::Fill),
        edit_button,
        delete_button
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .width(Length::Fill);

    container(row)
        .width(Length::Fill)
        .padding(15)
        .style(iced::theme::Container::Custom(Box::new(
            if todo.completed {
                ContainerStyle::CompletedTodoItem
            } else {
                ContainerStyle::TodoItem
            }
        )))
        .into()
} 