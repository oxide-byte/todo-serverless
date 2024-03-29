use leptos::RwSignal;
use crate::models::Todo;

pub mod app;
pub mod todo_modal;
pub mod todo_list_item;
pub mod todo_service;

pub type ShowTodoModalSignal = RwSignal<bool>;
pub type EditTodoSignal = RwSignal<Option<Todo>>;
pub type TodoSignal = RwSignal<Todo>;