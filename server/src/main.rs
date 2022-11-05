use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

fn run_queue(id: u32, messages: Arc<Mutex<Vec<String>>>) {
    println!("Spawning process {id}");
    tokio::spawn(async move {
        loop {
            let msg = {
                let mut msgs = messages.lock().unwrap();
                (*msgs).pop()
            };
            match msg {
                Some(v) => {
                    println!("Processed {v}by process {id}")
                }
                None => (),
            }
        }
    });
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let incoming_streams = listener.incoming();
    let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let total = 3;
    let (sender, receiver) = channel::<String>();

    for i in 0..total {
        run_queue(i as u32, Arc::clone(&messages));
    }

    tokio::spawn(async move {
        loop {
            let msg = receiver.recv().unwrap();
            let mut msgs = messages.lock().unwrap();
            (*msgs).push(msg);
        }
    });

    for stream in incoming_streams {
        let stream = stream.unwrap();
        handle_connection(stream, &sender);
    }
}

fn handle_connection(mut stream: TcpStream, sender: &Sender<String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let msg = String::from_utf8_lossy(&buffer[..])
        .trim_end_matches('\n')
        .to_string();
    println!(
        "{}",
        sender
            .send(msg.clone())
            .map(|_| "Added to queue")
            .unwrap_or("Something went wrong")
    );
}
