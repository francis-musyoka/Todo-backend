use actix_web::{
    Error, HttpResponse, Responder, get, post,
    web::{self, Data},
};
use chrono::Utc;

use crate::helpers::generate_id;
use crate::models::{CreateTodoItem, TodoItem, TodoResponse, TodoState, UpdateTodoItem};

#[post("/add/todo")]
pub async fn create_todo(
    items: web::Json<CreateTodoItem>,
    data: Data<TodoState>,
) -> Result<impl Responder, Error> {
    let mut todos = data
        .todo_list
        .lock()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock todo"))?;

    // Convert string am getting from generate_id to uuid.
    let id = generate_id();

    // Create new_todo
    let new_todo = TodoItem {
        id,
        title: items.title.clone(),
        completed: items.completed,
        created_at: Utc::now(),
        update_at: Utc::now(),
    };

    todos.push(new_todo.clone());

    Ok(HttpResponse::Created().json(TodoResponse {
        id: new_todo.id,
        title: new_todo.title,
        completed: new_todo.completed,
        created_at: new_todo.created_at,
    }))
}

//Get todos
#[get("/todos")]
pub async fn get_todo(data: Data<TodoState>) -> Result<impl Responder, Error> {
    let todos = data
        .todo_list
        .lock()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock todo"))?;
    Ok(HttpResponse::Ok().json(&*todos))
}

//Update todo
pub async fn update_todo(
    items: web::Json<UpdateTodoItem>,
    data: Data<TodoState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    // get all todo list
    let mut todos = data
        .todo_list
        .lock()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock todo"))?;

    let update_item = items.into_inner();

    // Get todo to be update by by matching todos id with provided id (path)
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *path) {
        if let Some(title) = update_item.title {
            todo.title = title;
        }
        if let Some(new_completed) = update_item.completed {
            todo.completed = new_completed;
        }

        todo.update_at = Utc::now();

        Ok(HttpResponse::Ok().json(todo))
    } else {
        Ok(HttpResponse::NotFound().body("Todo not found"))
    }
}

//delete todo
pub async fn delete_todo(
    data: Data<TodoState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let mut todos = data
        .todo_list
        .lock()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock todo"))?;

    let len_before = todos.len();
    todos.retain(|todo| todo.id != *path);
    let len_after = todos.len();

    if len_after < len_before {
        Ok(HttpResponse::Ok().json(&*todos))
    } else {
        Ok(HttpResponse::NotFound().body("Todo Not found"))
    }

    // if Some(pos) = todos.iter().position(|todo| todo.id == *path){
    //     todos.remove(pos);
    //     Ok(HttpResponse::Ok().json(&*todos))
    // }else {
    //     Ok(HttpResponse::NotFound().body("Todo Not found"))
    // }
}
