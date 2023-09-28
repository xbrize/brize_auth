use brize_auth::{
    application::SessionRepository,
    infrastructure::{handle_user_registration, DataStore},
};
use redis::{AsyncCommands, RedisResult};

#[tokio::main]
async fn main() -> RedisResult<()> {
    // let repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

    // let username = "test_name";
    // let password = "test_password";
    // let email = "test_email@gmail.com";

    // let session_id = handle_user_registration(username, password, email)
    //     .await
    //     .unwrap();
    // repo.get_session(&session_id).await.unwrap();

    let client = redis::Client::open("redis://:mypassword@localhost/").unwrap();
    let mut con = client.get_async_connection().await?;

    con.set("key1", b"foo").await?;

    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await;
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

    Ok(())
}
