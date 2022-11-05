use std::{fs, thread, time::Duration};

use async_std::{
    io::{ReadExt, WriteExt},
    net::{TcpListener, TcpStream},
    task,
};
use futures::StreamExt;
use http_server::ThreadPool;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let pool = ThreadPool::new(4);
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            handle_connection(stream).await;
        })
        .await;
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     // pool.execute(|| {
    //     handle_connection(stream).await;
    //     // });
    // }

    println!("Shutting down.");
}

static HTML_DIR: &str = "/home/arif/dev/rust-play/http-server/src/html";

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(10)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(format!("{HTML_DIR}/{filename}")).expect("Not found html");
    let length = contents.len();

    let content_length_header = format!("Content-Length: {length}");
    let response = format!("{status_line}\r\n{content_length_header}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
