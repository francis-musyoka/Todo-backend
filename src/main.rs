mod handlers;
mod helpers;
mod models;

use handlers::{
    create_todo, create_user, delete_todo, get_todo, log_in_user, update_todo, update_user_info,
    update_user_password,
};
use models::{TodoState, UserState};

use actix_cors::Cors;
use actix_web::{
    App, HttpServer, http,
    middleware::Logger,
    web::{self, delete, put},
};
use env_logger::Env;
use std::io;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let todo_state = web::Data::new(TodoState {
        todo_list: Mutex::new(Vec::new()),
    });

    let user_state = web::Data::new(UserState {
        user_list: Mutex::new(Vec::new()),
    });
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(user_state.clone())
            .app_data(todo_state.clone())
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            .wrap(cors)
            .service(get_todo)
            .service(create_todo)
            .service(
                web::resource("/todo/{id}")
                    .route(delete().to(delete_todo))
                    .route(put().to(update_todo)),
            )
            .service(create_user)
            .service(log_in_user)
            .service(update_user_info)
            .service(update_user_password)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
