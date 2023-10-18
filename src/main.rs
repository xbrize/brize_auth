use brize_auth::examples::{planet_scale_example, surreal_db_example};

#[tokio::main]
async fn main() {
    planet_scale_example().await.unwrap();
    surreal_db_example().await.unwrap();

    println!("Hello, Welcome to Brize Auth");
}
