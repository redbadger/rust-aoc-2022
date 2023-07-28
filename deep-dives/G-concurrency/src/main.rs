use std::{rc::Rc, sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{runtime::Runtime, sync::mpsc::Sender};

#[derive(Clone, Debug, serde::Deserialize)]
struct Body {
    args: Id,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Id {
    id: String,
}

fn url(id: u32) -> String {
    format!("https://postman-echo.com/get?id={id}")
}

fn blocking_request(id: u32) -> Result<()> {
    std::thread::sleep_ms(20000);
    let url = url(id);
    let response = reqwest::blocking::get(url)?;
    let body: Body = serde_json::from_str(&response.text()?)?;
    let body_id = body.args.id.parse::<u32>()?;
    assert_eq!(body_id, id);
    println!("{id}");
    Ok(())
}

fn series() -> Result<()> {
    for id in [1, 2, 3, 4, 5] {
        blocking_request(id)?
    }
    Ok(())
}

fn spawn_a_thread() {
    let id = 1234;

    let handle = std::thread::spawn(move || blocking_request(id));

    handle.join().unwrap();
}

fn parallel() -> Result<()> {
    // let mut handles = Vec::new();
    // for id in [1, 2, 3, 4, 5] {
    //     let handle = std::thread::spawn(move || blocking_request(id));
    //     handles.push(handle);
    // }

    let handles: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .map(|id| std::thread::spawn(move || blocking_request(id)))
        .collect();

    for h in handles {
        h.join().map_err(|_| anyhow::format_err!("join error!"))??;
    }
    Ok(())
}

async fn nonblocking_request(id: u32) -> Result<()> {
    let url = url(id);
    let response = reqwest::get(url).await?;
    let body: Body = serde_json::from_str(&response.text().await?)?;
    let body_id = body.args.id.parse::<u32>()?;
    assert_eq!(body_id, id);
    println!("{id}");
    Ok(())
}

async fn concurrent() {
    let request_futures: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .map(|id| nonblocking_request(id))
        .collect();
    futures::future::join_all(request_futures).await;
}

async fn nonblocking_request_with_sender(id: u32, sender: Sender<u32>) -> Result<()> {
    for _ in 0..3 {
        let url = url(id);
        let response = reqwest::get(url).await?;
        let body: Body = serde_json::from_str(&response.text().await?)?;
        let body_id = body.args.id.parse::<u32>()?;
        assert_eq!(body_id, id);
        sender.send(body_id).await;
    }
    Ok(())
}

async fn concurrent_with_sender(tx: Sender<u32>) {
    let request_futures: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .map(|id| nonblocking_request_with_sender(id, tx.clone()))
        .collect();
    futures::future::join_all(request_futures).await;
}

async fn concurrent_sender_runner() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let fut = concurrent_with_sender(tx);
    tokio::spawn(fut);
    while let Some(val) = rx.recv().await {
        println!("{val}")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // blocking_request(123);
    // series()?;
    // parallel()?;
    // spawn_a_thread();

    // concurrent().await;
    concurrent_sender_runner().await;
    Ok(())
}

fn no_send_rc() {
    let rc = Rc::new(String::from("hi"));
    // std::thread::spawn(move || rc); <-- BAD! Not thread safe
    let arc = Arc::new(String::from("hi"));
    std::thread::spawn(move || arc);
}

fn show_threadsafety() {
    threadsafety(123); // <- fine - has static lifetime

    let hello = String::from("hello");
    // threadsafety(&hello); <- BAD: not a static lifetime
}

fn threadsafety<T: Send + 'static>(val: T) {
    std::thread::spawn(move || val);
}
