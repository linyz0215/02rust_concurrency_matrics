use anyhow::Result;
use std::{sync::mpsc, thread, time::Duration};
const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i,tx));
    }
    drop(tx);
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exiting");
        42
    });
    let secret = consumer.join().unwrap();
    println!("The secret value is {}", secret);
    Ok(())
}

fn producer(idx: usize, tx:mpsc::Sender<Msg>)  -> Result<()> {
    loop {
        let value = rand::random::<u32>() as usize;
        tx.send(Msg::new(idx,value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exiting", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}