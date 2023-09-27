use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use brize_auth::{
    application::SessionRepository,
    infrastructure::{handle_user_registration, DataStore},
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

    // let username = "test_name";
    // let password = "test_password";
    // let email = "test_email@gmail.com";

    // let session_id = handle_user_registration(username, password, email)
    //     .await
    //     .unwrap();
    // repo.get_session(&session_id).await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
