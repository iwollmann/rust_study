use warp::Filter;
use warp::filters::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum WSMessage {
    Joinned { message: Option<String>},
    Join,
}

#[tokio::main]
async fn main() {
    let static_route = warp::path("static")
        .and(warp::fs::dir("www/static"));

    let hello = warp::path("ping")
        .map(|| "pong");

    let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| async move {
            let (mut tx, mut rx) = websocket.split();

             while let Some(result) = rx.next().await {
                let msg = match result {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("error reading message on websocket: {}", e);
                        break;
                    }
                };

                handle_message(msg, &mut tx).await;
             }
        })
    });

    let routes = static_route.or(hello).or(ws_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_message(msg: Message, sender: &mut SplitSink<WebSocket, Message>) {
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let received: WSMessage = serde_json::from_str(message).unwrap();

    let response = serde_json::to_string(&WSMessage::Joinned {
        message: Some("Welcome!".to_string())
    }).unwrap();

    match received {
        WSMessage::Join => sender.send(Message::text(response)).await.unwrap(),
        _ => ()
    }
}