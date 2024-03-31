use postgres::{Client, NoTls};
use potsgres::Error as PostgresError;
use std::env;
use std::io::{BufReader, Read, Write};
use std::new::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

const DB_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

const HOST: &str = "0.0.0.0";
const PORT: &str = "8080";
const ENDPOINT: String = HOST.to_owned() + ":" + PORT;

fn main() {
    if let Err(_) = create_database() {
        println!("Error setting database");
        return;
    }

    let listener =
        TcpListener::bind(ENDPOINT.clone()).expect("Error binding {HOST} to port: {PORT}");
    println!("SERVER LISTENING PORT:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
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
        OK(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
        }
    }
}

fn create_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute("CREATE EXTENSION pg_oauth IF NOT EXISTS");
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
