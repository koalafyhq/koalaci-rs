extern crate redis;

pub fn connect_redis() -> redis::Connection {
  let redis_client = redis::Client::open("redis://127.0.0.1/").expect("cannot open redis client");
  let redis = redis_client
    .get_connection()
    .expect("cannot connect to redis server");

  redis
}
