use mini_redis::{client, Result};

// #[warn(dead_code)]
// async fn say_world()
// {
//     println!("world");
// }

#[tokio::main]
async fn main() -> Result<()>{
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result = {:?}", result);

    // let op = say_world(); // op.await才真正的调用
    // println!("hello");
    // op.await;

    Ok(())
}
