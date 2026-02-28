use tokio::sync::mpsc::{Receiver, Sender, channel, error::SendError};

pub struct TaskSender<T> {
    inner: Sender<T>,
}

pub struct TaskReceiver<T> {
    inner: Receiver<T>,
}

pub fn bounded<T>(capacity: usize) -> (TaskSender<T>, TaskReceiver<T>) {
    let (tx, rx) = channel(capacity);

    (TaskSender { inner: tx }, TaskReceiver { inner: rx })
}

impl<T> TaskSender<T> {
    pub async fn send(&self, task: T) -> Result<(), SendError<T>> {
        self.inner.send(task).await
    }
}

impl<T> TaskReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.inner.recv().await
    }
}
