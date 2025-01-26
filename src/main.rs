
use std::sync::{Arc, Mutex};
use std::thread;


// Define a simple structure to encapsulate the pool's state
struct Pool {
    usdt_amount: u64,
    dot_amount: u64,
    k: u64,
}

mod kfk_sw_producer;
use kfk_sw_producer::producer;

 fn main() {

   //mt_liquidty_pool()
   producer::kafka_producer()
}



use tokio::time::{sleep, Duration};

#[tokio::main]
async fn tokyotest() {

    let pool = Arc::new(Mutex::new(Pool { 
        usdt_amount: 100_000_000, 
        dot_amount: 10_000_000, 
        k: 100_000_000 * 10_000_000 
    }));

    let mut handles = vec![];

    for _ in 0..10 {
        let pool_clone = Arc::clone(&pool);
        handles.push(tokio::spawn(async move {
            for i in 0..100 { // Loop 5 times for demonstration
                let thread_id = std::thread::current().id();
                let swap_amount = (rand::random::<f64>() * 1000.0) as u64;
                let dot_or_usdt = rand::random::<f64>() < 0.25;

                if let Ok(mut pool) = pool_clone.lock() {
                    if dot_or_usdt {
                        println!("Buying {} DOT", swap_amount);
                        pool.dot_amount -= swap_amount;
                        pool.usdt_amount = pool.k / pool.dot_amount;
                    } else {
                        println!("Buying {} USDT", swap_amount);
                        pool.usdt_amount -= swap_amount;
                        pool.dot_amount = pool.k / pool.usdt_amount;
                    }
                    println!("DOT: {}, USDT: {}, K: {}", pool.dot_amount, pool.usdt_amount, pool.k);
                    println!("{:?} - USDT/DOT: {}", thread_id,pool.usdt_amount as f64 / pool.dot_amount as f64);
                } else {
                    println!("Failed to acquire lock");
                }

                sleep(Duration::from_millis(100)).await; // Small delay to see the output
            }
        }))
    }
   
    for handle in handles {
        handle.await.unwrap();
    }
}


fn mt_liquidty_pool() {
    println!("Liquidity pool simulation with threads");

    let pool = Arc::new(Mutex::new(Pool { 
        usdt_amount: 100_000_000, 
        dot_amount: 10_000_000, 
        k: 100_000_000 * 10_000_000 
    }));

    let mut handles = vec![];

    // Spawn 10 threads for example. You can adjust this number
    for _ in 0..10 {
        let pool_clone = Arc::clone(&pool);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            for _ in 0..100 {  // Each thread will make 100 swaps
                let swap_amount = (rand::random::<f64>() * 1000.0) as u64;
                let dot_or_usdt = rand::random::<f64>() < 0.25;

                if let Ok(mut pool) = pool_clone.lock() {
                    if dot_or_usdt {
                        println!("Buying {} DOT", swap_amount);
                        pool.dot_amount -= swap_amount;
                        pool.usdt_amount = pool.k / pool.dot_amount;
                    } else {
                        println!("Buying {} USDT", swap_amount);
                        pool.usdt_amount -= swap_amount;
                        pool.dot_amount = pool.k / pool.usdt_amount;
                    }
                    println!("DOT: {}, USDT: {}, K: {}", pool.dot_amount, pool.usdt_amount, pool.k);
                    println!("{:?} - USDT/DOT: {}", thread_id,pool.usdt_amount as f64 / pool.dot_amount as f64);
                } else {
                    println!("Failed to acquire lock");
                }
                // Add a small sleep to prevent overwhelming the console output
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }));
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let pool = pool.lock().unwrap(); // Here we don't care about the lifetime issue because we're at the end of main
    println!("Final state: DOT: {}, USDT: {}, K: {}", pool.dot_amount, pool.usdt_amount, pool.k);
    println!("Final USDT/DOT: {}", pool.usdt_amount as f64 / pool.dot_amount as f64);

}