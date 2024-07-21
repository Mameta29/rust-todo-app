use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_postgres::NoTls;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTodoRequest {
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateTodoRequest {
    completed: bool,
}

async fn create_connection_pool(database_url: &str) -> tokio_postgres::Client {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls)
        .await
        .expect("Failed to create connection");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

async fn get_todos(client: web::Data<tokio_postgres::Client>) -> HttpResponse {
    match client
        .query("SELECT id, title, completed FROM todos ORDER BY id", &[])
        .await
    {
        Ok(rows) => {
            let todos: Vec<Todo> = rows
                .iter()
                .map(|row| Todo {
                    id: row.get(0),
                    title: row.get(1),
                    completed: row.get(2),
                })
                .collect();
            HttpResponse::Ok().json(todos)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_todo(
    client: web::Data<tokio_postgres::Client>,
    todo: web::Json<CreateTodoRequest>,
) -> HttpResponse {
    match client
        .query_one(
            "INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING id, title, completed",
            &[&todo.title, &todo.completed],
        )
        .await
    {
        Ok(row) => {
            let created_todo = Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get(2),
            };
            HttpResponse::Created().json(created_todo)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn update_todo(
    client: web::Data<tokio_postgres::Client>,
    id: web::Path<i32>,
    todo: web::Json<UpdateTodoRequest>,
) -> HttpResponse {
    println!("Updating todo: id={}, completed={}", id, todo.completed); // デバッグ用

    match client
        .execute(
            "UPDATE todos SET completed = $1 WHERE id = $2",
            &[&todo.completed, &id.into_inner()],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(format!("Error: {}", e))
        }
    }
}

async fn delete_todo(
    client: web::Data<tokio_postgres::Client>,
    id: web::Path<i32>,
) -> HttpResponse {
    match client
        .execute("DELETE FROM todos WHERE id = $1", &[&id.into_inner()])
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let client = create_connection_pool(&database_url).await;
    let client_data = web::Data::new(client);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(client_data.clone())
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
