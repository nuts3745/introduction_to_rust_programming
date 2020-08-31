use futures::executor;

fn main() {
    async fn async_add(left: i32, right: i32) -> i32 {
        left + right
    }

    async fn something_great_async_function() -> i32 {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }

    executor::block_on(something_great_async_function());
}
