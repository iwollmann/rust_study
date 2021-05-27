use warp::{Filter, hyper::Uri};
use warp::filters::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum WSMessage {
    Joinned { message: Option<String>},
    Join { room: String, id: u32 },
}

#[tokio::main]
async fn main() {
    let static_route = warp::path("static")
        .and(warp::fs::dir("www/static"));

    let hello = warp::path("ping")
        .map(|| "pong");

    let create_room = warp::path::end()
        .map(|| {
            let room_id = Uuid::new_v4();
            warp::redirect(format!("/room/{}", room_id).parse::<Uri>().unwrap())
        });

    let room = warp::path("room")
        .and(warp::path::param())
        .map(|room_id: String| {
            let body = format!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>WebRTC Study</title>
                <meta charset="UTF-8" />
                <link href="/static/styles.css" rel="stylesheet" />
                <script>
                    const ROOM_ID = "{}"
                </script>
                <script src="/static/index.js" defer></script>
            </head>
            <body>
                <div id="video-container"></div>
                <!-- <button id="enable-audio">Enable audio</button> -->
            </body>
            </html>
            "#, room_id);
            warp::reply::html(body)
        });

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

    let routes = static_route
        .or(hello)
        .or(create_room)
        .or(room)
        .or(ws_route);
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
        WSMessage::Join { room, id} => sender.send(Message::text(response)).await.unwrap(),
        _ => ()
    }
}