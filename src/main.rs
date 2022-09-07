use std::{net::{TcpListener, TcpStream}, process, sync::{Arc, Mutex}, io::Write};

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
println!("READ 1");
    // TODO: Do checks for Player struct, like if the name is already used
    match packet.mut_recieved_from().as_mut() {
        Some(player) => player,
        None => {
            eprintln!("Player not provided");
            // write error to stream
            return;
        }
    }.set_turn(*turn.lock().unwrap()); // Set the players turn

    // Set players initial cards (Unwrap is safe here since we already know the player field exists)
    packet.mut_recieved_from().as_mut().unwrap().set_cards((*game.lock().unwrap()).draw_hand());

    // Add player to the vector of players
    (*game.lock().unwrap()).add_player(&packet.recieved_from().as_ref().unwrap());

    let id = packet.recieved_from().as_ref().unwrap().id().clone();

    // Send id to client
    stream.write_all(format!("{}\r\n", id).as_bytes()).unwrap();
println!("WRITE 1");
    // Clear out the recieved from to save a bit of memory
    *packet.mut_recieved_from() = None;

    loop {
        // Clone Arc<Mutex<Game>> in a Game, and send the Game structure to the client-
        *packet.game_mut() = Some((*game.lock().unwrap()).clone());
        // Write the packet
        packet.write(&mut stream).unwrap();

        // Keep reading the card until we get a valid one
        loop {
            // Recieve card
            packet = Packet::read(&mut stream).unwrap();
            // if the card does not match
            if !packet.game().as_ref().unwrap().card_matches(&packet.card().as_ref().unwrap()) {
                packet.set_error(Some(String::from("That card does not work on the current card!")));
                packet.write(&mut stream).unwrap();
                continue;
            }
            
            // Remove card from player
            // set card to current card

            break;
        }

        // Write recieved game to Arc<Mutex<Game>>
        // Next player turn
    }
}
