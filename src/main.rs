use std::{error::Error, sync::Arc};

use brio_smart_tech::{BrioSmartTech, Color};
use btleplug::{api::Manager as _, platform::Manager};
use tokio::{
    sync::{
        Mutex,
        broadcast::{Receiver, Sender, channel},
    },
    time::{Duration, sleep},
};
use train_box::rpi_io::RpiIo;

#[derive(Debug, Clone)]
enum Message {
    Disconnected,
    Forward,
    Backward,
    ChangeColor,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rpi_io = RpiIo::new()?;

    println!("Initializing BLE manager");
    let manager = Manager::new().await.unwrap();
    // Get the first bluetooth adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.first().unwrap();

    loop {
        println!("Searching for train");
        rpi_io.led_disconnected_set_high().await;
        rpi_io.led_connected_set_low().await;
        let train = Arc::new(Mutex::new(
            BrioSmartTech::new(central)
                .await?
                .expect("device not found"),
        ));
        println!("Device found");
        let (tx, rx) = channel(100);
        let tx = Arc::new(tx);
        let bf_tx = tx.clone();

        rpi_io
            .button_forward_set_async_interrupt(move |_event| {
                bf_tx.send(Message::Forward).unwrap();
            })
            .await?;

        let bb_tx = tx.clone();
        rpi_io
            .button_backward_set_async_interrupt(move |_event| {
                bb_tx.send(Message::Backward).unwrap();
            })
            .await?;

        rpi_io.led_disconnected_set_low().await;
        rpi_io.led_connected_set_high().await;
        // let _= tokio::task::spawn(watch_connected(train.clone(),
        // tx.clone())).await;

        let _ = manage_train(train, rx)
            .await
            .inspect_err(|e| println!("{e}"));
        println!("Disconnected");
    }
}

// TODO Need a task that watches that we are still connected to the device to
// unlock manage_train loop.
async fn watch_connected(
    train: Arc<Mutex<BrioSmartTech>>,
    tx: Arc<Sender<Message>>,
) {
    loop {
        let res = train.lock().await.is_connected().await;

        if res.is_err() || res.unwrap() {
            tx.send(Message::Disconnected).unwrap();
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }
}

async fn manage_train(
    train: Arc<Mutex<BrioSmartTech>>,
    mut rx: Receiver<Message>,
) -> Result<(), Box<dyn Error>> {
    enum State {
        Stopped,
        Forward,
        Backward,
    }

    let mut state = State::Stopped;

    loop {
        let message = rx.recv().await.inspect_err(|e| eprintln!("{e}"));

        if message.is_err() {
            break;
        }

        let message = message.unwrap();

        match message {
            Message::Forward => {
                println!("Forward pressed");
                if matches!(state, State::Forward) {
                    train.lock().await.stop().await?;
                    state = State::Stopped;
                } else {
                    train.lock().await.forward(7).await?;
                    state = State::Forward;
                }
            }
            Message::Backward => {
                println!("Backward pressed");
                if matches!(state, State::Backward) {
                    train.lock().await.stop().await?;
                    state = State::Stopped;
                } else {
                    train.lock().await.backward(7).await?;
                    state = State::Backward;
                }
            }
            Message::ChangeColor => (),
            Message::Disconnected => return Ok(()),
        }
    }

    Ok(())
}
