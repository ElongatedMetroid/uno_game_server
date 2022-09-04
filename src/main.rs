use std::{net::{TcpListener, TcpStream}, process, sync::{Arc, Mutex}};

mod threadpool;

use lib_uno_game::Game;
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else( |error| {
        eprintln!("Could not bind to address: {error}");
        process::exit(1);
    });

    let pool = ThreadPool::build(5).unwrap();

    let game = Game::new();
    let game = Arc::new(Mutex::new(game));

    for stream in listener.incoming() {
        let mut turn = 0;
        let game = Arc::clone(&game);

        pool.execute(move || {
            handle_client(stream.unwrap(), game, turn);
        });

        turn += 1;
    }
}

fn handle_client(stream: TcpStream, game: Arc<Mutex<Game>>, turn: usize) {
    // Recieve initial data, like name


    player.set_turn(turn);

    // game.add_player(recieved.player);

    // When all players are ready send cards to each

    loop {
        // Clone Arc<Mutex<Game>> in a Game, and send the Game structure to the client-

        // Recieve card
        // (check if its valid)

        // Next player turn
    }
}
