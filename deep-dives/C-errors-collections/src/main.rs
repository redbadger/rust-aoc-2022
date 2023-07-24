// Result defined as:
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// fn main() {
//     let response = can_error_basic().unwrap();
//     println!("{response}");
// }

fn can_error_basic() -> Result<String, String> {
    if rand::random::<f64>() > 0.5 {
        Ok(String::from("all is well"))
    } else {
        Err(String::from("unlucky"))
    }
}

type AnyError = Box<dyn std::error::Error>;

fn can_error_explicit() -> Result<String, AnyError> {
    let res = if rand::random::<f64>() > 0.5 {
        Ok(String::from("all is well"))
    } else {
        Err(String::from("unlucky"))
    };
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(AnyError::from(e)),
    }
}

fn can_error_try_operator() -> Result<String, AnyError> {
    if rand::random::<f64>() > 0.5 {
        Ok(String::from("all is well"))
    } else {
        // Err(String::from("unlucky")) // type error
        Err("unlucky")? // works because of implicit `.into()` call
    }
}

use anyhow::Result as AnyResult;

// AnyResult defined as:
// type AnyResult<T> = Result<T, anyhow::Error>;

use std::io::Read;

fn lots_of_error() -> AnyResult<()> {
    let mut file = std::fs::File::open("some file")?;

    // let mut file = match std::fs::File::open("some file") {
    //     Ok(ok) => ok,
    //     Err(e) => return Err(anyhow::anyhow!("{e}")),
    // };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    "hello world".parse::<i32>()?;

    serde_json::from_str("not valid json")?;

    anyhow::bail!("something went wrong");
}

fn main() -> Result<(), anyhow::Error> {
    println!("this is stdout");
    eprintln!("this is stderr");
    Ok(())
    // lots_of_error()
}
