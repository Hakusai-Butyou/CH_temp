use redis::Client;

pub async fn create_redis_client() -> Client {
    Client::open("redis://127.0.0.1/").unwrap()
}