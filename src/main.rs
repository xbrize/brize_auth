use surreal_auth::models::user;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("test").use_db("test").await?;

    // let new_user = user::User::new("myusrname", "mypassword", "myemail");
    // user::init_user_table(&db).await?;
    // user::register_user(&db, new_user).await?;
    // let login_status = user::login_user(&db, "myusrname", "mypassword").await;
    // dbg!(login_status);

    Ok(())
}
