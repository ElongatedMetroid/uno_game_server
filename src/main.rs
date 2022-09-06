use std::{net::{TcpListener, TcpStream}, process, sync::{Arc, Mutex}};

mod threadpool;

use lib_uno_game::{Game, Packet};
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else( |error| {
        eprintln!("Could not bind to address: {error}");
        process::exit(1);
    });

    let pool = ThreadPool::build(5).unwrap();

    let game = Arc::new(Mutex::new(Game::new()));
    let turn = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        let game_clone = Arc::clone(&game);
        let turn_clone = Arc::clone(&turn);

        pool.execute(|| {
            handle_client(stream.unwrap(), game_clone, turn_clone);
        });
        *turn.lock().unwrap() += 1;
    }
}

fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>, turn: Arc<Mutex<usize>>) {
    // Recieve initial data, like name
    let mut packet = match Packet::read(&mut stream) {
        Ok(packet) => packet,
        
        Err(error) => {
            eprintln!("Error was most likely clients fault: {error}");
            // write error to stream
            return;
        }
    };

    // TODO: Do checks for Player struct, like if the name is already used
    match packet.mut_recieved_from().as_mut() {
        Some(player) => player,
        None => {
            eprintln!("Player not provided");
            // write error to stream
            return;
        }
    }.set_turn(*turn.lock().unwrap()); // Set the players turn

    // Add player to the vector of players
    (*game.lock().unwrap()).add_player(&packet.recieved_from().as_ref().unwrap());

    println!("{:#?}", game);

    // When all players are ready send cards to each
    loop {
        // Clone Arc<Mutex<Game>> in a Game, and send the Game structure to the client-

        // Recieve card
        // (check if its valid, return Error response if not)

        // Write game
        // Next player turn
    }
}
