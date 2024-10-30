use core::str;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?; // 指定のアドレスに対してリッスン用のソケットを作成
    loop {
        let (mut stream, _) = listener.accept()?; // 2 メインスレッドでTCP接続を待機
        thread::spawn(move || { // 3 処理を実行するスレッドを新規作成 stream の所有権がメインスレッドから move される
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap(); // 4 
                if nbytes == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
