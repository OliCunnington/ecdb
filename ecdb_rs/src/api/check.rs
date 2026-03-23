use axum::{
    extract::Path,
    extract::State, 
    response::IntoResponse, 
    Json
};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use surrealdb::RecordId;
use surrealdb::opt::auth::Record;
use crate::{
    state::AppState,
};

pub async fn health_check_alive() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn health_check_ready(State(app_state): State<AppState>) -> impl IntoResponse {
    // Simple database connectivity check
    match app_state.db.health().await {
        Ok(_) => Json(json!({ 
            "status": "ready", 
            "database": "connected" 
        })),
        Err(_) => Json(json!({ 
            "status": "not ready", 
            "database": "disconnected" 
        })),
    }
}

mod error {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use axum::Json;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db,
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db
        }
    }
}

// basic handler that responds with a static string
pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn session(State(app_state): State<AppState>) -> Json<Value> {
    match app_state.db.query("<string>$session").await {
        Ok(x) => {
            println!("session: {x:?}");
            Json(json!({"message":"{x:?}"}))
        },
        Err(x) => Json(json!({"message":"Error"}))
    }
}

pub async fn get_session_ac(State(app_state): State<AppState>) -> Json<Value> {
        match app_state.db.query("RETURN session::ac()").await {
            Ok(x) => {
                println!("session_ac: {x:?}");
                // Json(json!(x.take(0))) //.unwrap()))
                Json(json!({}))
            },
            Err(x) => Json(json!({
                "status": "Error",
                "error" : x.to_string()
            }))
        }
}

#[derive(Serialize, Deserialize)]
struct Person {
    first: String,
    middle: String,
    last: String
}

pub async fn get_customers(State(app_state): State<AppState>) -> Json<Value> {
    match app_state.db.query("SELECT VALUE name FROM customer").await {
        Ok(mut x) => Json(json!(x.take::<Vec<Person>>(0))),
        Err(_) => Json(json!({ 
            "status": "not ready", 
            "database": "disconnected" 
        }))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: RecordId,
    name: String,
    stock: i32,
    price: f64,
    description: String,
    volume: i32,
    categorys: Vec<String>,
    supplied_by: Option<String>,
    reviews: Vec<String>,
    average_rating: f64,
}

pub async fn get_products(State(app_state): State<AppState>) -> Json<Value> {
    match app_state.db.query("SELECT * FROM product").await {
        Ok(mut x) => {
            let r : Vec<Product> = x.take(0).unwrap();
            Json(json!(r))
        },
        Err(_) => Json(json!({ 
            "status": "not ready", 
            "database": "disconnected" 
        }))
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignupData {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SigninData {
    email: String,
    password: String,
}



pub async fn sign_up(State(app_state): State<AppState>, Json(payload): Json<SignupData>) -> Json<Value> { //Result<surrealdb::opt::auth::Jwt, surrealdb::Error> {//Json<User> {
    
    tracing::info!("Signup : {}", serde_json::to_string(&payload).unwrap());
    let r =  app_state.db.signup(Record {
                namespace: "main",
                database: "ecdb",
                access: "account",
                params: payload
                // params: SignupData {
                //     name: payload.name,
                //     email: payload.email,
                //     password: payload.password,
                // }
            })
            .await;
    tracing::info!("Result r: {:?}", r);
    match r {
                Ok(x) => {
                    // println!("{x:?}");
                    tracing::info!("{x:?}");
                    Json(json!({
                        "token" : x.into_insecure_token()
                    }))
                },
                Err(x) => {
                    tracing::info!("{x:?}");
                    Json(json!({ 
                        "status": "error", 
                        "database": "unable to signup" 
                    }))
                }
            }
}

pub async fn sign_in(State(app_state): State<AppState>, Json(payload): Json<SigninData>) -> Json<Value> { //Result<surrealdb::opt::auth::Jwt, surrealdb::Error> {
    tracing::info!("Signin : {}", serde_json::to_string(&payload).unwrap());
    match app_state.db.signin(Record {
                namespace: "main",
                database: "ecdb",
                access: "account",
                params: payload
                // params: SigninData {
                //     email: payload.email,
                //     password: payload.password,
                // }
            })
            .await {
                Ok(x) => {
                    tracing::info!("Ok: {x:?}");
                    Json(json!({
                        "token" : x.into_insecure_token()
                    }))
                },
                Err(x) => { 
                    tracing::info!("Error: {x:?}");
                    Json(json!({ 
                        "status": "error", 
                        "database": "unable to signup" 
                    }))
                }
            }
}

pub async fn sign_out(State(app_state): State<AppState>) -> Json<Value> {
    // yup... signs out the root access it seems
    match app_state.db.invalidate().await {
                Ok(x) => {
                    tracing::info!("{x:?}");
                    Json(json!({
                        "message": "signed out"
                    }))
                },
                Err(x) => { 
                    tracing::info!("{x:?}");
                    Json(json!({ 
                        "status": "error", 
                        "database": "unable to sign out" 
                    }))
                }
            }
}

pub async fn get_product(State(app_state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    // let prod : Result<Option<Product>, _> = app_state.db.select(("product", &*id)).await.take(0);
    // match prod {
    //     Ok(x) => {
    //         Json(json!(x))
    //     },
    //     Err(x) => {
    //         tracing::info!("{x:?}");
    //         Json(json!({
    //             "message":"product not found"
    //         }))
    //     }
    // }
    tracing::info!("fetching product : {id:?}");
    let prod : Result<Option<Product>, _> = app_state.db.select(("product", &*id)).await;
    tracing::info!("{prod:?}");
    Json(json!(prod))
}