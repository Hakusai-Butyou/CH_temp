#[cfg(feature = "ssr")]
use redis::Client;

#[cfg(feature = "ssr")]
pub async fn create_redis_client() -> Client {
    Client::open("redis://:20080130@redis:6379/").unwrap()
}