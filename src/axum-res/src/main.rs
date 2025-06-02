use axum::{
    routing::get,
    Json, 
    Router,
    extract::State,
    http::StatusCode,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

type SharedUserList = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    let users = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(get_users).post(add_user))
        .route("/users/:id", get(get_user_by_id))
        .with_state(users);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler: GET /
async fn hello() -> &'static str {
    "Hello from Axum! Try GET /users or POST /users"
}

// Handler: GET /users
async fn get_users(State(users): State<SharedUserList>) -> Json<Vec<User>> {
    let users = users.lock().unwrap();
    Json(users.clone())
}

// Handler: POST /users
async fn add_user(
    State(users): State<SharedUserList>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = users.lock().unwrap();
    
    // Generate new ID (simple auto-increment)
    let new_id = users.len() as u32 + 1;
    
    // Create new user
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    
    // Add to the list
    users.push(new_user.clone());
    
    println!("Added user: {:?}", new_user);
    Ok(Json(new_user))
}

// Handler: GET /users/:id
async fn get_user_by_id(
    State(users): State<SharedUserList>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<Json<User>, StatusCode> {
    let users = users.lock().unwrap();
    
    match users.iter().find(|user| user.id == id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}