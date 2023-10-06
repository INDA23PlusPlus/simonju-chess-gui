mod state;
mod networking;
mod board;

use std::thread;

use networking::Player;
use state::*;
use ggez::*;

/* Todo
 * I must handle the game state: check, checkmate, fifty rule limit etc.
 */

fn main() -> GameResult {
    println!("CHESS");
    println!("Choose Gamemode:");
    println!("\tl - local"); 
    println!("\ts - server");
    println!("\tc - client");

    loop {
        let mut option = String::new();
        std::io::stdin().read_line(&mut option)?;
        let char = option.to_ascii_lowercase().chars().next();
        option.clear();
        match char {
            Some('l') => { 
                println!("Starting local game");
                run_local()?;
                break;
            },
            Some('s') => {
                println!("Choose port:");
                std::io::stdin().read_line(&mut option)?;
                let port = option.trim().to_string();
                run_server(port)?;
                break;
            },
            Some('c') => {
                println!("Choose address:");
                std::io::stdin().read_line(&mut option)?;
                let address = option.trim().to_string();
                option.clear();

                println!("Choose player (w, b):");
                std::io::stdin().read_line(&mut option)?;
                let player = match option.to_ascii_lowercase().chars().next() {
                    Some('w') => Player::White,
                    Some('b') => Player::Black,
                    _ => {
                        println!("Invalid input!");
                        continue;
                    }
                };
                run_client(player, address)?;
                break;
            },
            _ => {
                println!("Invalid input!");
                continue;
            },
        };
    }

    Ok(())
}

fn run_local() -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("chess", "simonju")
        .add_resource_path(std::path::PathBuf::from("./config"))
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()?;

    let state = State::new(&mut context)?;
    
    event::run(context, event_loop, state);
}

fn run_client(player: Player, addr: String) -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("chess", "simonju")
        .add_resource_path(std::path::PathBuf::from("./config"))
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()?;

    let state = State::new_client(&mut context, player, addr)?;

    event::run(context, event_loop, state);
}

fn run_server(port: String) -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("chess", "simonju")
        .add_resource_path(std::path::PathBuf::from("./config"))
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()?;

    let state = State::new_server(&mut context, port)?;

    event::run(context, event_loop, state);
}