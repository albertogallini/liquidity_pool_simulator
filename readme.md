# Liquidity Pool Simulator

This is a simple Rust program that simulates a liquidity pool using threads, the Tokio crate, and Kafka. To use the Kafka producer & consumer, you'll need a Kafka broker and Zookeeper up and running. 
<br>I suggest running Docker to start a Zookeeper instance and a Kafka broker in separate containers.

## Setting Up Kafka with Docker

Use the following commands to ensure that your Kafka broker is listening on the correct port and that a topic with the correct name is created:

- **Start Zookeeper Instance:**
```bash
docker run -d --name zookeeper -p 2181:2181 -e ZOOKEEPER_CLIENT_PORT=2181 wurstmeister/zookeeper
```

Start Kafka Broker:

```bash
docker run -d --name kafka --link zookeeper:zookeeper -p 9092:9092 -e KAFKA_ADVERTISED_HOST_NAME=localhost -e KAFKA_ADVERTISED_PORT=9092 -e KAFKA_ZOOKEEPER_CONNECT=zookeeper:2181 wurstmeister/kafka
```

Create the Topic:

```bash
docker exec -it kafka kafka-topics.sh --create --zookeeper zookeeper:2181 --replication-factor 1 --partitions 1 --topic test-topic
```
Monitor Topic Content:

```bash
docker exec -it kafka kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic test-topic --from-beginning
```
