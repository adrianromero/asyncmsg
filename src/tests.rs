//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2023 Adrián Romero Corchado.

use super::*;
use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
struct Message(i32);

#[tokio::test]
async fn multi_receiver() {
    let (tx, rx) = mpsc::channel::<Message>(10);
    tx.send(Message(1)).await.unwrap();
    tx.send(Message(2)).await.unwrap();
    tx.send(Message(3)).await.unwrap();
    tx.send(Message(4)).await.unwrap();
    tx.send(Message(5)).await.unwrap();

    let rx = MultiReceiver::from(rx);
    let h1_rx = rx.clone();
    let h2_rx = rx.clone();

    assert_eq!(h1_rx.recv().await, Some(Message(1)));
    assert_eq!(h1_rx.recv().await, Some(Message(2)));
    assert_eq!(h2_rx.recv().await, Some(Message(3)));

    drop(tx);

    assert_eq!(h2_rx.recv().await, Some(Message(4)));
    assert_eq!(h1_rx.recv().await, Some(Message(5)));
    assert_eq!(h2_rx.recv().await, None);
    assert_eq!(h1_rx.recv().await, None);
}
