use mini_redis::client;
use std::thread;
use std::time::Duration;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    thread::sleep(Duration::from_secs(5));
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    tokio::pin!(messages); // 让数据pin住，意味着其不可以被 move

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

async fn easy_stream() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(x) = stream.next().await {
        println!("GOT {:?}", x)
    }
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;
    println!("DONE");
    Ok(())
}
