use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    println!("Connected to chat server. Type messages and press Enter to send:");

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        if let Err(e) = ws_stream.send(Message::text(input)).await {
                            eprintln!("Error sending message: {e}");
                            break;
                        }
                    }
                    Ok(None) => {
                        println!("Goodbye!");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {e}");
                        break;
                    }
                }
            }
            
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(text) = msg.as_text() {
                                println!("{text}");
                            }
                        } else if msg.is_close() {
                            println!("Server closed the connection");
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message: {e}");
                        break;
                    }
                    None => {
                        println!("Server disconnected");
                        break;
                    }
                }
            }
        }
    }

    let _ = ws_stream.close().await;
    Ok(())
}