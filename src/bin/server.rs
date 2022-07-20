use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::{sync::{Arc, Mutex}, collections::HashMap, hash};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type SharedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main()
{
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _)  = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        // process(socket).await;
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }

    // let shared = db[hash(key) % db.len()].lock().unwrap();
    // shared.insert(key, value);
}

async fn process(socket: TcpStream, db: Db)
{
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

fn new_shared_db(num_shares: usize) -> SharedDb {
    let mut db = Vec::with_capacity(num_shares);
    for _ in 0..num_shares {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}