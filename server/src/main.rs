use dotenv::dotenv;
use server::ThreadPool;

use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main() {
    dotenv().ok();
    const HOST: &str = "0.0.0.0";
    const PORT: &str = "8080";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    if create_database().is_err() {
        println!("Error setting database");
        return;
    }

    let listener =
        TcpListener::bind(endpoint.clone()).expect("Error binding {HOST} to port: {PORT}");
    println!("SERVER LISTENING PORT:8080");

    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(e) => pool.execute(move || {
                println!("Unable to connect: {}", e);
            }),
        }
    }
}

/// Reads TCP stream into a buffer and matches it to the appropriate endpoint.
///
/// if reading from the stream was successful, convert the buffer to a UTF-8 encoded string and append it to the request string.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /api/rust/") => (
                    OK_RESPONSE.to_string(),
                    String::from("Welcome to the R3 Rust API"),
                ),
                _ => (NOT_FOUND.to_string(), String::from("404 NOT FOUND")),
            };
            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

fn create_database() -> Result<(), PostgresError> {
    let db_url: String = std::env::var("DATABASE_URL").unwrap_or_default();
    println!("{db_url}");
    let mut client = Client::connect(&db_url, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ",
    )?;
    Ok(())
}
