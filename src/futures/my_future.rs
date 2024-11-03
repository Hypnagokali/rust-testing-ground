use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

pub struct MyFuture {
    progress: Arc<Mutex<Progress>>,
}

impl MyFuture {
    pub fn new() -> Self {
        let progress = Arc::new(Mutex::new(Progress {
            message: "started".to_string(),
            done: false,
            waker: None
        }));

        let res_for_thread = progress.clone();
        thread::spawn(move || {
            // just mock some async work
            thread::sleep(Duration::from_secs(2));
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = "Import data ...".to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            thread::sleep(Duration::from_secs(1));
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = "Group data ...".to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            thread::sleep(Duration::from_secs(2));
            if let Ok(mut state) = res_for_thread.lock() {
                state.done = true;
                state.message = "I am done !!!".to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
        });

        MyFuture {
            progress,
        }
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

impl Future for MyFuture {
    type Output = MyResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Do poll");
        let mut current = self.progress.lock().unwrap();
        if current.done {
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

fn do_something() -> MyFuture {
    println!("Call do_something");
    MyFuture::new()
}


#[tokio::test]
async fn test_future () {
    println!("start test");
    let result = do_something().await;

    assert_eq!(result.result, "Done");
}