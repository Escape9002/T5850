use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = connect_to_server()?;

    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let msg = line?;

        handle_message(&msg, &mut stream)?;
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

fn handle_message(cmd: &str, stream: &mut TcpStream) -> std::io::Result<()> {
    println!("[MSG]\t{}", cmd);
    let package: Vec<&str> = cmd.split('|').collect();


    match package[0] {
        "tick" => {
            stream.write_all(b"move|right\n")?;
            // Maybe one could run some commands to react here?
        }
        "pos" => {
            println!("Position update");
            // or store game-info
        }
        "game" => {
            println!("Game start");
            // or start sending things
        }
        "lose" => {
            println!("Game over");
            // or reset game info
        }
        _ => {
            println!("No reaction");
        }
    }


    Ok(())
}
