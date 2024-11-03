use std::future::Future;
use std::pin::Pin;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

pub struct ImportFuture {
    progress: Arc<Mutex<Progress>>,
}

impl ImportFuture {
    const IMPORT_DATA: &'static str = "Import data ...";
    const GROUP_DATA: &'static str = "Group data ...";
    const DONE: &'static str = "Done \u{1F643}";

    pub fn new() -> Self {
        let progress = Arc::new(Mutex::new(Progress {
            message: "started".to_string(),
            done: false,
            waker: None
        }));

        let res_for_thread = progress.clone();
        thread::spawn(move || {
            let (sender, receiver) = tokio::sync::mpsc::channel::<String>(10);
            // just mock some async work
            thread::sleep(Duration::from_secs(2));
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = Self::IMPORT_DATA.to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            thread::sleep(Duration::from_secs(1));
            if let Ok(mut state) = res_for_thread.lock() {
                state.message = Self::GROUP_DATA.to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
            thread::sleep(Duration::from_secs(2));
            if let Ok(mut state) = res_for_thread.lock() {
                state.done = true;
                state.message = Self::DONE.to_string();
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
        });

        ImportFuture {
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

fn do_something() -> ImportFuture {
    println!("Call do_something");
    ImportFuture::new()
}


#[tokio::test]
async fn test_future () {
    println!("start test");
    let result = do_something().await;

    assert_eq!(result.result, "Done");
}