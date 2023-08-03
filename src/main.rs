
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::error::Error;
use std::time::Duration;

pub async fn produce(brokers: &str, topic_name: &str, message: &str) -> Result<(i32, i64),  Box<dyn Error>> {
  let producer: &FutureProducer = &ClientConfig::new()
    .set("bootstrap.servers", brokers)
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  let res: (i32, i64) = match producer.send(
  FutureRecord::to(topic_name)
    .payload(message)
    .key("simple-key"),
    Duration::from_secs(0),
  ).await {
    Ok(v) => v,
    Err(err) => return Err(Box::new(err.0)),
  };

  Ok(res)
}

#[tokio::main]
async fn main() {
    println!("Starting produce");
    produce("127.0.0.1:9092", "timeout_test", "foo").await.unwrap();
    println!("End produce");
}

