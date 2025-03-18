use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoState {
    todos: HashMap<usize, Todo>,
    next_id: usize,
    filter: Filter,
    search_query: String,
    editing: Option<(usize, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddTodo,
    DeleteTodo(usize),
    ToggleTodo(usize),
    FilterChanged(Filter),
    SearchQueryChanged(String),
    EditingTodo(usize, String),
    InputChanged(String),
    DescriptionChanged(usize, String),
    FinishEditing,
    CancelEditing,
}

impl Default for TodoState {
    fn default() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
            filter: Filter::All,
            search_query: String::new(),
            editing: None,
        }
    }
}

impl TodoState {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddTodo => {
                if let Some((_, title)) = &self.editing {
                    if !title.trim().is_empty() {
                        let id = self.next_id;
                        self.todos.insert(
                            id,
                            Todo {
                                id,
                                title: title.clone(),
                                description: String::new(),
                                completed: false,
                                created_at: Local::now(),
                            },
                        );
                        self.next_id += 1;
                        self.editing = Some((0, String::new()));
                    }
                } else {
                    self.editing = Some((0, String::new()));
                }
            }
            Message::DeleteTodo(id) => {
                self.todos.remove(&id);
                if let Some((editing_id, _)) = self.editing {
                    if editing_id == id {
                        self.editing = None;
                    }
                }
            }
            Message::ToggleTodo(id) => {
                if let Some(todo) = self.todos.get_mut(&id) {
                    todo.completed = !todo.completed;
                }
            }
            Message::FilterChanged(filter) => {
                self.filter = filter;
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
            }
            Message::EditingTodo(id, current_text) => {
                self.editing = Some((id, current_text));
            }
            Message::InputChanged(text) => {
                if let Some((id, _)) = self.editing {
                    self.editing = Some((id, text));
                }
            }
            Message::DescriptionChanged(id, text) => {
                if let Some(todo) = self.todos.get_mut(&id) {
                    todo.description = text;
                }
            }
            Message::FinishEditing => {
                if let Some((id, title)) = &self.editing {
                    let title = title.trim();
                    if *id > 0 && !title.is_empty() {
                        if let Some(todo) = self.todos.get_mut(id) {
                            todo.title = title.to_string();
                        }
                    }
                    self.editing = None;
                }
            }
            Message::CancelEditing => {
                self.editing = None;
            }
        }
    }

    pub fn search_query(&self) -> &String {
        &self.search_query
    }
    
    pub fn filter(&self) -> &Filter {
        &self.filter
    }

    pub fn filtered_todos(&self) -> Vec<&Todo> {
        let mut todos: Vec<&Todo> = self.todos.values().collect();
        
        // Apply filter
        todos.retain(|todo| match self.filter {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        });
        
        // Apply search if there's a query
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            todos.retain(|todo| {
                todo.title.to_lowercase().contains(&query)
                    || todo.description.to_lowercase().contains(&query)
            });
        }
        
        // Sort by creation date (newest first)
        todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        todos
    }

    pub fn editing(&self) -> Option<(usize, &String)> {
        self.editing.as_ref().map(|(id, text)| (*id, text))
    }

    fn data_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("todo_gui");
        fs::create_dir_all(&path).ok();
        path.push("todos.json");
        path
    }

    pub async fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize todos: {}", e))?;
        
        fs::write(Self::data_path(), json)
            .map_err(|e| format!("Failed to save todos: {}", e))
    }

    pub async fn load() -> Result<Self, String> {
        let path = Self::data_path();
        
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let data = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read todo file: {}", e))?;
        
        serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse todos: {}", e))
    }
} 