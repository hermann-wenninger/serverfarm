use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

// Diese Funktion wird in einem eigenen Thread für jeden Client ausgeführt
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); // Eingehende Anfrage lesen

    println!("Anfrage empfangen: {}", String::from_utf8_lossy(&buffer[..]));

    // Eine simple HTTP-Antwort erstellen
    let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hallo von Rust!</h1>";

    stream.write(response.as_bytes()).unwrap(); // Antwort senden
    stream.flush().unwrap(); // Sicherstellen, dass alles gesendet wurde
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server lauscht auf http://127.0.0.1:8080");

    // Endlosschleife, um Verbindungen anzunehmen
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Für jede erfolgreiche Verbindung einen neuen Thread starten
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Verbindung fehlgeschlagen: {}", e);
            }
        }
    }
}