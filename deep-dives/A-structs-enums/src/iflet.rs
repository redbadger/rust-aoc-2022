fn do_if_let() {
    let res: Result<String, String> = Err("something went wrong".into());

    match res {
        Ok(_) => todo!(),
        Err(e) => println!("{e}"),
    }

    let opt: Option<&str> = Some("got the thing");

    match opt {
        Some(it) => println!("got it: {it}"),
        None => {}
    }

    if let Some(it) = opt {
        println!("got it: {it}")
    }

    let more = More::C(10);

    if let More::C(val) = more {
        println!("{val}")
    }
}

enum More {
    A,
    B,
    C(u8),
}

fn while_let() {
    let elems = vec![1, 2, 3, 4, 5, 6];

    let mut iter = elems.iter();

    while let Some(val) = iter.next() {
        println!("{val}")
    }

    // equivalent to...
    for val in iter {
        println!("{val}")
    }
}

fn let_else() {
    let it: Option<&str> = Some("hi");

    let Some(it) = it else { return };

    // let Some(thing) = res else { return Err("new thing") }

    // same as
    // let it = if let Some(it) = it { it } else { return };

    println!("{it}")
}
