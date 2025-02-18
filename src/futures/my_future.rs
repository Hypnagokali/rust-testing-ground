use std::future::Future;
use std::pin::Pin;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
pub struct ImportFuture {
    progress: Arc<Mutex<Progress>>,
}

impl ImportFuture {
    const IMPORT_DATA: &'static str = "Import data ...";
    const GROUP_DATA: &'static str = "Group data ...";
    const DONE: &'static str = "Done \u{1F643}";

    pub fn new() -> (Self, Receiver<String>) {
        let mut current_message = "none".to_string();
        let progress = Arc::new(Mutex::new(Progress {
            message: "started".to_string(),
            done: false,
            waker: None
        }));

        let res_for_thread = progress.clone();
        let (sender, receiver) = tokio::sync::mpsc::channel::<String>(10);
        tokio::spawn(async move {
            // just mock some async work
            tokio::time::sleep(Duration::from_secs(2)).await;
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = Self::IMPORT_DATA.to_string();
                current_message = state.message.clone();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            if let Ok(_) = sender.send(current_message.clone()).await {
                println!("Sender sends message: import");
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = Self::GROUP_DATA.to_string();
                current_message = state.message.clone();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            if let Ok(_) = sender.send(current_message.clone()).await {
                println!("Sender sends message: group");
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
            if let Ok(mut state) = res_for_thread.lock() {
                state.done = true;
                state.message = Self::DONE.to_string();
                current_message = state.message.clone();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            if let Ok(_) = sender.send(current_message.clone()).await {
                println!("Sender sends message: done");
            }
        });

        (ImportFuture {
            progress,
        }, receiver)
    }
}

struct Progress {
    message: String,
    done: bool,
    waker: Option<Waker>,
}

pub struct MyResult {
    result: String,
}

impl Future for ImportFuture {
    type Output = MyResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Do poll");
        let mut current = self.progress.lock().unwrap();
        if current.done {
            println!("All done. Message = {}", current.message);
            Poll::Ready(MyResult {
                result: "Done".to_string()
            })
        } else {
            println!("Not done yet. Message = {}", current.message);
            current.waker = Some(cx.waker().clone());
            Poll::Pending
        }

    }
}

pub fn do_something() -> (ImportFuture, Receiver<String>) {
    println!("Call do_something");
    ImportFuture::new()
}

mod tests {
    use std::time::Duration;
    use tokio::try_join;
    use crate::futures::my_future::do_something;

    #[tokio::test]
async fn test_future () {
    println!("start test");
    let fut_rec = do_something();

    let fut = fut_rec.0;
    let mut receiver = fut_rec.1;

    let h1 = tokio::spawn(async move {
        fut.await
    });

    let h2 = tokio::spawn(async move {
        // busy wait for testing
        loop {
            match receiver.recv().await {
                Some(message) => {
                    println!("Received message: {:?}", message);
                    if message.starts_with("Done") {
                        break;
                    }
                }
                None => {
                    println!("Nothing received.");
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    match try_join!(h1, h2) {
        Ok((result1, _)) => {
            assert_eq!(result1.result, "Done");
        }
        Err(_) => {
            panic!("Received an Err");
        }
    }

}

}
