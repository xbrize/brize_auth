use surreal_auth::models::user;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("test").use_db("test").await?;

    // user::create_user_table(db).await?;
    // user::create_user(db, user::User::new("maname", "paxzword", "pazzz@eamil.com")).await?;
    // let user = user::read_user(db, "maname").await;
    // dbg!(user);
    Ok(())
}
