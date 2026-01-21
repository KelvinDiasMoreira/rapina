use rapina::extract::State;
use rapina::prelude::*;

#[derive(Clone)]
struct AppConfig {
    app_name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[get("/")]
async fn hello(config: State<AppConfig>) -> String {
    format!("Hello from {}!", config.into_inner().app_name)
}

#[get("/health")]
async fn health() -> StatusCode {
    StatusCode::OK
}

#[get("/users/:id")]
async fn get_user(id: Path<u64>) -> Result<Json<User>> {
    let id = id.into_inner();

    if id == 0 {
        return Err(Error::bad_request("id cannot be zero"));
    }

    if id == 999 {
        return Err(Error::not_found("user not found"));
    }

    Ok(Json(User {
        id,
        name: "Antonio".to_string(),
        email: "antonio@tier3.dev".to_string(),
    }))
}

#[post("/users")]
async fn create_user(body: Json<CreateUser>) -> Json<User> {
    let input = body.into_inner();
    Json(User {
        id: 1,
        name: input.name,
        email: input.email,
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig {
        app_name: "Rapina Demo".to_string(),
    };

    let router = Router::new()
        .get("/", hello)
        .get("/health", health)
        .get("/users/:id", get_user)
        .post("/users", create_user);

    Rapina::new()
        .state(config)
        .router(router)
        .listen("127.0.0.1:3000")
        .await
}
