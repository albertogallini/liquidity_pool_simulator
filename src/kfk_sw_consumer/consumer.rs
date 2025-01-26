use rdkafka::config::ClientConfig;
use rdkafka::consumer::{ Consumer, StreamConsumer};
use ::futures::StreamExt;
use rdkafka::{ Message};


use std::sync::{Arc, Mutex};


struct Pool {
    usdt_amount: u64,
    dot_amount: u64,
    k: u64,
}



async fn kafka_consumer(pool: Arc<Mutex<Pool>>, client_id: &str) {
    // Configure the consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092") // Adjust this to your Kafka broker's address
        .set("group.id", "my_group")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("client.id", client_id)
        .create()
        .expect("Consumer creation error");

    // Subscribe to one or more topics
    consumer.subscribe(&["test-topic"]).unwrap();

    // Create a stream from the consumer
    let mut stream = consumer.stream();
    
    println!("consumer {:?} - Waiting for messages...", client_id);

    loop {
        // Await the next message from the stream
        match stream.next().await {
            Some(Ok(m)) => {        
                println!(
                    "Received message on partition {} at offset {}: {}",
                    m.partition(),
                    m.offset(),
                    String::from_utf8_lossy(m.payload().unwrap())
                );
                
                if let Some(payload) = m.payload() {
                    let payload_str = String::from_utf8_lossy(payload);
                    if let Some((currency, amount)) = payload_str.split_once(':') {
                        let currency: &str = currency;
                        if let Ok(amount) = amount.trim().parse::<f64>() {
                            println!("Currency: {}, Amount: {}", currency, amount);
                            let mut pool = pool.lock().unwrap();
                            match currency {
                                "USDT" => {
                                    pool.usdt_amount = (pool.usdt_amount as f64 + amount) as u64;
                                    pool.k = pool.usdt_amount * pool.dot_amount;
                                    println!("Updated USDT amount: {}", pool.usdt_amount);
                                },
                                "DOT" => {
                                    pool.dot_amount = (pool.dot_amount as f64 + amount) as u64;
                                    pool.k = pool.usdt_amount * pool.dot_amount;
                                    println!("Updated DOT amount: {}", pool.dot_amount);
                                },
                                _ => println!("Unknown currency: {}", currency),
                            }
                            // Store or process currency and amount as needed
                        } else {
                            println!("Failed to parse amount: {}", amount);
                        }
                    } else {
                        println!("Message format invalid: {}", payload_str);
                    }
                }
            },
            Some(Err(e)) => {
                println!("Error receiving message: {:?}", e);
            },
            None => {
                println!("Stream ended unexpectedly");
                break; // Note: In practice, you might want to handle this differently
            }
        }
    }
}



#[tokio::main]
pub async fn consume() {

    let pool = Arc::new(Mutex::new(Pool { 
        usdt_amount: 100_000_000, 
        dot_amount: 10_000_000, 
        k: 100_000_000 * 10_000_000 
    }));

    let mut handles = vec![];

    for i in 0..10 {
        let pool_clone = Arc::clone(&pool);
        let client_id = format!("client_{}", i);
        handles.push(tokio::spawn(async move {
                kafka_consumer(pool_clone,&client_id).await;
            }
        ));
    };
        
   
    for handle in handles {
        handle.await.unwrap();
    }
}