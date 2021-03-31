use memory_db::MemoryDb;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command, Connection, Frame};
mod memory_db;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    let db = MemoryDb::new();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: MemoryDb) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match db.process(Command::from_frame(frame).unwrap()) {
            Ok(res) => match res {
                memory_db::CmdResult::Get(get_res) => get_res.value.map(Frame::Bulk).unwrap_or(Frame::Null),
                memory_db::CmdResult::None => Frame::Simple("OK".to_string())
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}