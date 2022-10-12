use tokio::sync::oneshot;

async fn some_operation() -> String {
    String::from("one")
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }

            _ = tx1.closed() => {
                println!("tx1 channel closed")
            }
        }
    });

    tokio::spawn(async move {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }

        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}
