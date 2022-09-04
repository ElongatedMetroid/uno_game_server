use std::{net::{TcpListener, TcpStream}, process, sync::{Arc, Mutex}};

mod threadpool;
mod game;

use threadpool::ThreadPool;
use game::Game;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else( |error| {
        eprintln!("Could not bind to address: {error}");
        process::exit(1);
    });

    let pool = ThreadPool::build(5).unwrap();

    let game = Game::new();
    let game = Arc::new(Mutex::new(game));

    for stream in listener.incoming() {
        let game = Arc::clone(&game);

        pool.execute(|| {
            handle_client(stream.unwrap(), game);
        });
    }
}

fn handle_client(stream: TcpStream, game: Arc<Mutex<Game>>) {
    // game.add_player(recieved.player);

    loop {
        // Clone Arc<Mutex<Game>> in a Game, and send the Game structure to the client
    }
}
