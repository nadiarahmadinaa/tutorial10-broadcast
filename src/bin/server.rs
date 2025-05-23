use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("WebSocket connection established: {addr}");
    
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(text) = msg.as_text() {
                                println!("Received from {addr}: {text}");

                                let _ = bcast_tx.send(format!("{addr}: {text}"));
                            }
                        } else if msg.is_close() {
                            println!("Client {addr} disconnected");
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message from {addr}: {e}");
                        break;
                    }
                    None => {
                        println!("Client {addr} disconnected");
                        break;
                    }
                }
            }
            
            broadcast_msg = bcast_rx.recv() => {
                match broadcast_msg {
                    Ok(msg) => {
                        if !msg.starts_with(&format!("{addr}:")) {
                            if let Err(e) = ws_stream.send(Message::text(msg)).await {
                                eprintln!("Error sending message to {addr}: {e}");
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }

    println!("Connection handler for {addr} ended");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("listening on port 8081");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}