use futures::executor;

struct User {
    // 何らかのデータ
}

struct UserId(u32);

struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {
        // DBに接続するなどの実装
    }
}

async fn find_by_user_id(db: Db, user_id: UserId) -> Option<User> {
    // db はデータベースアクセスを示す。
    db.find_by_user_id(user_id).await
}

fn main() {
    executer::block_on(find_by_user_id(Db {}, UserId(1)));
}
