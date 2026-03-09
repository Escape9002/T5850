pub mod game;

use std::{env, vec};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::game::{Coordinates, GameState, mark, print_map, tick};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = connect_to_server()?;

    let mut game = GameState{ board: vec![vec![0]], my_id: 0 };
    
    loop {
        let reader = BufReader::new(stream.try_clone()?);
        
        for line in reader.lines() {
            let msg = line?;
    
            handle_message(&msg, &mut stream, &mut game)?;
        }
    }

    Ok(())
}

fn connect_to_server() -> Result<TcpStream, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let ip = env::var("SERVER_IP")?;
    let port = env::var("SERVER_PORT")?;
    let bot = env::var("BOT_NAME")?;
    let key = env::var("BOT_KEY")?;

    let addr = format!("{ip}:{port}");

    let mut stream = TcpStream::connect(&addr)?;

    let join = format!("join|{}|{}\n", bot, key);
    stream.write_all(join.as_bytes())?;

    Ok(stream)
}

fn handle_message(cmd: &str, stream: &mut TcpStream, game: &mut GameState) -> std::io::Result<()> {
    println!("[MSG]\t{}", cmd);
    let package: Vec<&str> = cmd.split('|').collect();

    match package[0] {
        "tick" => {
            // stream.write_all(b"move|right\n")?;
            // Maybe one could run some commands to react here?
            let stppedity  = tick(&game);

            let cmd = format!("move|{}\n",  stppedity.as_str());

            stream.write_all(cmd.as_bytes())?;

        }
        "pos" => {
            println!("Position update");
            let x = package[1].parse().unwrap();
            let y = package[2].parse().unwrap();
            let id = package[3].parse().unwrap();

            mark(Coordinates{x,y}, id, &mut game.board);
            
        }
        "game" => {
            println!("Game start");
            // or start sending things
            let width= package[1].parse().unwrap();
            let heigth = package[2].parse().unwrap();
            let my_id = package[3].parse().unwrap();

            let board = vec![vec![0; width]; heigth];
            
            
            game.board = board;
            game.my_id = my_id;
            
        }
        "lose" => {
            println!("Game over");
            // or reset game info
            game.board = vec![vec![0]; 0];
        }
        _ => {
            println!("No reaction");
        }

    }
    
    print_map(&game.board);

    Ok(())
}


