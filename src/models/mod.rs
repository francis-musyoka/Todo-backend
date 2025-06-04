pub mod todo;
pub mod user;

use std::sync::Mutex;
pub use todo::{CreateTodoItem, TodoItem, TodoResponse, UpdateTodoItem};
pub use user::{
    CreateUser, CreateUserResponse, LogInUser, UpdateUserInfo, UpdateUserPassword, User,
    UserLoginResponse,
};

pub struct TodoState {
    pub todo_list: Mutex<Vec<TodoItem>>,
}

pub struct UserState {
    pub user_list: Mutex<Vec<User>>,
}
