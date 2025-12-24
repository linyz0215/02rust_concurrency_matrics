use anyhow::Result;
use concurrency::Metrics;
use std::{thread, time::Duration};
use rand::Rng;
const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone());//Metrics {data: Arc::clone(&metrics))};
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop{
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    }); 
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..80)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}