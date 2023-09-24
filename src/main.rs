use surreal_auth::models::session::{create_session, get_session};
use surreal_auth::models::{get_user, user};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("test").use_db("test").await?;

    // user::init_user_table(&db).await?;
    let new_user = user::User::new("test-username", "test-password", "cassie@email.com");
    // user::create_user(&db, &new_user).await?;
    // user::register_user(&db, &new_user).await;
    // let login_status = user::login_user(&db, "myusrname", "mypassword").await;
    // dbg!(login_status);

    let user = get_user(&db, "cassie@email.com").await;
    let new_session = create_session(&db, user.unwrap().id).await;
    let sesh = get_session(&db, new_session.unwrap()).await;
    dbg!(sesh);

    Ok(())
}
