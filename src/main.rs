mod lua_engine;
mod crypto_data;
mod ui_bridge;
use ui_bridge::{ui_bridge,UIStore};
mod config;
use std::str::FromStr;
use futures::{SinkExt, StreamExt};
use i_slint_backend_winit::WinitWindowAccessor;
use serde::__private::de::IdentifierDeserializer;
use slint::{ComponentHandle, SharedString};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let UIStore {bg,t} = ui_bridge();
    let url = "wss://advanced-trade-ws.coinbase.com";

    let (mut ws_stream, response) = connect_async(url).await.expect("Failed to connect");

    let subscribe_message = r#"{
        "type": "subscribe",
        "product_ids": ["BTC-USD"],
        "channel": "ticker"
    }"#;

    ws_stream
        .send(Message::Text(subscribe_message.to_string().into()))
        .await?;

    let main_window = MainWindow::new().unwrap();
    main_window
        .window()
        .set_position(slint::LogicalPosition::new(0.0, 0.0));
    let ui_handle = main_window.as_weak();
    
    // make main_window draggable
    /*    main_window.on_mouse_move(move |delta_x, delta_y| {
        let ui_handle = ui_handle.unwrap();
        let logical_pos = ui_handle.window().position().to_logical(ui_handle.window().scale_factor());
        ui_handle.window().set_position(slint::LogicalPosition::new(logical_pos.x + delta_x,logical_pos.y + delta_y))
    }

    );*/

    tokio::spawn(async move {
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let parsed: serde_json::Value =
                        serde_json::from_str(&text).expect("Can't parse to JSON");
                    if let Some(events) = parsed.get("events").and_then(|e| e.as_array()) {
                        for event in events {
                            if let Some(tickers) = event.get("tickers").and_then(|t| t.as_array()) {
                                for ticker in tickers {
                                    if let Some(price) = ticker.get("price") {
                                        let price_string = price.as_str().unwrap_or("0.0");
                                        let price_string = SharedString::from(price_string);
                                        ui_handle
                                            .clone()
                                            .upgrade_in_event_loop(move |ui| {
                                                ui.set_bit_price(price_string);
                                            })
                                            .unwrap();
                                        println!("BTC Price: {}", price);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    });

    main_window.set_background_color(bg);
    main_window.set_text_color(t);
    main_window.run();
    Ok(())
}
