use serde::{Serialize, Deserialize};
use serde_json::json;
use surrealdb::{Error, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    db.use_ns("main").use_db("main").await?;

    let info = db.query("INFO FOR DB").await?;

    dbg!(info);

    Ok(())
}
