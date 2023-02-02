//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2023 Adrián Romero Corchado.

use super::*;
use tokio::sync::mpsc;

#[tokio::test]
async fn multi_receiver() {
    let (tx, rx) = mpsc::channel::<i32>(10);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    tx.send(3).await.unwrap();
    tx.send(4).await.unwrap();
    tx.send(5).await.unwrap();

    let rx = MultiReceiver::from(rx);
    let h1_rx = rx.clone();
    let h2_rx = rx.clone();

    assert_eq!(h1_rx.recv().await, Some(1));
    assert_eq!(h1_rx.recv().await, Some(2));
    assert_eq!(h2_rx.recv().await, Some(3));

    drop(tx);

    assert_eq!(h2_rx.recv().await, Some(4));
    assert_eq!(h1_rx.recv().await, Some(5));
    assert_eq!(h2_rx.recv().await, None);
    assert_eq!(h1_rx.recv().await, None);
}
