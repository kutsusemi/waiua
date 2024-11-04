use valorant_local::apis;

async fn hoge() {
    let res =
        apis::default_api::chat_v4_friends_get(&apis::configuration::Configuration::default())
            .await;
    println!("{:?}", res);
}
// ClientをFacadeとして使う
// ログインできるまで使えないようにしたい
// 各種状態の時のstructを作る
struct Client {}
impl Client {
    fn new() -> Self {
        Self {}
    }
}
struct State {}
