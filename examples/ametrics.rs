use anyhow::Result;
use concurrency::AmapMetrics;
use std::{thread, time::Duration};
use rand::Rng;
const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = AmapMetrics::new(
        &[
            "call.thread.worker.0",
            "call.thread.worker.1",
            "req.page.1",
            "req.page.2",
            "req.page.3",
            "req.page.4",
        ]
    );

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

fn task_worker(idx: usize, metrics: AmapMetrics) {
    thread::spawn(move || loop{
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    }); 
}

fn request_worker(metrics: AmapMetrics) -> Result<()> {
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