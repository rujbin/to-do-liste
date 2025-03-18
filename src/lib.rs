use iced::{
    Application, Command, Element, Theme, executor,
};
use todo::TodoState;
use ui::view;

pub mod todo;
pub mod ui;

pub struct TodoApp {
    todo_state: TodoState,
}

#[derive(Debug, Clone)]
pub enum Message {
    TodoMessage(todo::Message),
    LoadTodos,
    TodosLoaded(Result<TodoState, String>),
    SaveTodos,
    TodosSaved(Result<(), String>),
}

impl Application for TodoApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                todo_state: TodoState::default(),
            },
            Command::perform(async {}, |_| Message::LoadTodos),
        )
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TodoMessage(msg) => {
                self.todo_state.update(msg);
                Command::perform(async {}, |_| Message::SaveTodos)
            }
            Message::LoadTodos => {
                Command::perform(TodoState::load(), Message::TodosLoaded)
            }
            Message::TodosLoaded(Ok(state)) => {
                self.todo_state = state;
                Command::none()
            }
            Message::TodosLoaded(Err(_)) => {
                // If loading fails, we'll just keep the default state
                Command::none()
            }
            Message::SaveTodos => {
                let cloned_state = self.todo_state.clone();
                Command::perform(async move { cloned_state.save().await }, Message::TodosSaved)
            }
            Message::TodosSaved(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        view::view_app(&self.todo_state).map(Message::TodoMessage)
    }
} 