
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use tokio::time::Duration;

#[tokio::main]
pub async fn kafka_producer() {
    // Configure the producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092") // Adjust this to your Kafka broker's address
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Send a message
    for _ in 0..100 { // Loop 5 times for demonstration
        let swap_amount = (rand::random::<f64>() * 1000.0) as u64;
        let dot_or_usdt = rand::random::<f64>() < 0.25;
        let mut payload_str = String::from("");
        if dot_or_usdt {
            payload_str.push_str("DOT:");
        } else {
            payload_str.push_str("USDT:");
        }
        
        payload_str.push_str(&swap_amount.to_string());

        let delivery_status = producer
        .send(
            FutureRecord::to("test-topic")
                .payload(payload_str.to_string().as_bytes())
                .key("key1"),
            Duration::from_secs(0),
        )
        .await;

        match delivery_status {
            Ok((partition, offset)) => println!(
                "Message sent to partition {} at offset {}",
                partition, offset
            ),
            Err((e, _)) => println!("Error delivering message: {:?}", e),
        }

        println!("Using librdkafka version: {:?}", get_rdkafka_version());

    }

    
    
}