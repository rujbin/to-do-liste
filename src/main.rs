use iced::{Application, Command, Element, Settings, Theme, Error};
use todo_gui::TodoApp;

// Keep the hide_console function for Windows
#[cfg(windows)]
use winapi::um::wincon::GetConsoleWindow;
#[cfg(windows)]
use winapi::um::winuser::ShowWindow;
#[cfg(windows)]
use winapi::um::winuser::SW_HIDE;

#[cfg(windows)]
fn hide_console() {
    unsafe {
        let window = GetConsoleWindow();
        if !window.is_null() {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn main() -> Result<(), Error> {
    // Hide console window on Windows
    #[cfg(windows)]
    hide_console();
    
    TodoApp::run(Settings {
        window: iced::window::Settings {
            size: (800, 600),
            position: iced::window::Position::Centered,
            min_size: Some((400, 300)),
            ..Default::default()
        },
        ..Default::default()
    })
} 