//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2023 Adrián Romero Corchado.

use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

pub struct MultiReceiver<T> {
    rx: Arc<Mutex<Receiver<T>>>,
}
impl<T> From<Receiver<T>> for MultiReceiver<T> {
    fn from(rx: Receiver<T>) -> Self {
        MultiReceiver {
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}
impl<T> Clone for MultiReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
        }
    }
}
impl<T> MultiReceiver<T> {
    pub async fn recv(&self) -> Option<T> {
        let mut rx_guard = self.rx.lock().await;
        rx_guard.recv().await
    }
}

#[cfg(test)]
mod tests;
