use server::thread::ThreadPool;

use core::panic;
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    email: String,
    user_name: String,
    name: String,
    desc: Option<String>,
    profile_image_url: Option<String>,
}

const DB_URL: &str = std::env!("DATABASE_ARGS");
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main() {
    const HOST: &str = "0.0.0.0";
    const PORT: &str = "8080";

    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let listener =
        TcpListener::bind(endpoint.clone()).expect("ERR: binding {HOST} to port: {PORT} failed");
    println!("GO: SERVER LISTENING {HOST}:{PORT}");

    let pool = ThreadPool::new(8);

    match create_database() {
        Ok(_) => (),
        Err(err) => println!("ERR: Database failed to create {}", err),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(err) => pool.execute(move || {
                eprintln!("ERR: Could not parse stream, {}", err);
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
            request.push_str(&String::from_utf8_lossy(&buffer[..size]));

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /api/") => (
                    OK_RESPONSE.to_string(),
                    String::from("Welcome to the R3 Rust API"),
                ),
                r if r.starts_with("POST /api/auth/github") => handle_github_login(r),

                _ => (NOT_FOUND.to_string(), String::from("404 NOT FOUND")),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }

        Err(e) => eprintln!("ERR: Unable to read stream: {}", e),
    }
}

fn create_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(&DB_URL, NoTls)?;

    let create_query = r#"
        CREATE TABLE IF NOT EXISTS "users" (
            "id" VARCHAR(191) NOT NULL,
            "email" VARCHAR(191) NOT NULL,
            "user_name" VARCHAR(20) NOT NULL,
            "name" VARCHAR(20) NOT NULL,
            "desc" VARCHAR(500),
            "profile_image_url" VARCHAR(191),
            PRIMARY KEY ("id"),
            CONSTRAINT "users_email_key" UNIQUE ("email"),
            CONSTRAINT "users_userName_key" UNIQUE ("user_name")
        )
    "#;

    match client.batch_execute(create_query) {
        Ok(_) => {
            println!("OK: All database table creation queries executed successfully.");
            Ok(())
        }
        Err(err) => panic!("ERR: Error executing database queries: {}", err),
    }
}

fn get_request_body(req: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(req.split("\r\n\r\n").last().unwrap_or_default())
}

fn handle_github_login(req: &str) -> (String, String) {
    println!("OK: Received GitHub login request: \r\n{}", req);

    match get_request_body(&req) {
        Ok(user) => println!(
            "OK: GitHub login request body successfully parsed: \r\n{:?}",
            user
        ),
        Err(err) => panic!(
            "ERR: GitHub login request body could not be parsed successfully {}",
            err
        ),
    };

    (OK_RESPONSE.to_string(), req.to_string())
    // Code to be inserted
}
