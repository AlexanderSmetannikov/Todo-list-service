use axum::{
    routing::{get, post}, Router, extract::State
};
use std::net::SocketAddr;
use axum_error::Result;
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqlitePool;
use axum::extract::Json;
use tower_http::cors::CorsLayer;
use axum::Form;
use axum::extract::Path;
use axum::response::Redirect;


#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", post(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Serialize, Deserialize)]
struct Todo {
    task_id: i64,
    description: Option<String>, 
    done: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    description: String, 
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String, 
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Shared {
    id: i64, 
    task_id: i64,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT task_id, description, done FROM tasks ORDER BY task_id"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(todos))
}

async fn create_user(State(pool): State<SqlitePool>, Form(user): Form<User>) -> Result<()> {
    sqlx::query!("INSERT INTO users (username, password) VALUES (?, ?)", user.username, user.password).execute(&pool).await?;
    Ok(())
}

// async fn authenticate_user(State(pool): State<SqlitePool>, Form(credentials): Form<User>) -> Result<Json<bool>> {
//     let user = sqlx::query!(
//         r#"SELECT * FROM users WHERE username = ? AND password = ?"#,
//         credentials.username,
//         credentials.password
//     )
//     .fetch_optional(&pool)
//     .await?;

//     let is_valid = user.is_some();

//     Ok(Json(is_valid))
// }



async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    sqlx::query!("INSERT INTO tasks (description) VALUES (?)", todo.description).execute(&pool).await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    sqlx::query!("DELETE FROM tasks WHERE task_id = ?", id).execute(&pool).await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    sqlx::query!("UPDATE tasks SET description = ?, done = ? WHERE task_id = ?", todo.description, todo.done, todo.task_id).execute(&pool).await?;
    Ok(Redirect::to("http://localhost:5173"))
}
