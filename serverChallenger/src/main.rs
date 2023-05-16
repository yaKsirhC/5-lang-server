use reqwest;
use std::fs;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::io::Write;


const NUM_REQUESTS: usize = 10000;
const CONCURRENT_REQUESTS: usize = 1000000;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error + Send + Sync> >{
    let start_time = Instant::now();

    let client = reqwest::Client::new();
    let requests = (0..NUM_REQUESTS).map(|_i| {
        let client = client.clone();
        tokio::spawn(async move {
            let url = "http://localhost:9002/";
            let response = client.get(url).send().await?;
            let _ = response.text().await?;  // Discard the response body
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
    });

    let mut completed_requests = 0;
    for request in requests {
        if completed_requests >= CONCURRENT_REQUESTS {
            sleep(Duration::from_secs(100)).await;
            completed_requests = 0;
        }
        request.await??;
        completed_requests += 1;
        println!("{}", completed_requests)
    }

    let elapsed_time = start_time.elapsed();
    let avg_request_time = elapsed_time / NUM_REQUESTS as u32;
    println!("Elapsed time: {:?}", elapsed_time);
    println!("Average request time: {:?}", avg_request_time);
    let strj = format!("Average request time: {:?}\n", avg_request_time);
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open("results.txt")
      .unwrap();
      
    file.write_all(&strj.as_bytes());

    Ok::<(),Box<dyn std::error::Error + Send + Sync>>(())
}