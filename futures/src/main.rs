use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    return left + right
}

async fn hoge() -> i32 {
    let num = async_add(2, 3).await;
    println!("{}", num);
    return num
}

fn main() {
    executor::block_on(hoge()); // async fnを実行するのに必要
}
