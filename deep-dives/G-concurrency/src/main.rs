use anyhow::Result;

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

fn main() -> Result<()> {
    series()?;
    parallel()?;

    Ok(())
}
