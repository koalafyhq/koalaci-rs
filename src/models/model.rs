use crate::models::Job;
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<redis::Connection>>;

#[derive(Clone)]
pub struct Model {
  job: Job,
}

impl Model {
  pub fn new() -> Model {
    // TODO: change this to ENV
    let redis_host = "redis://127.0.0.1/";
    let redis_client = redis::Client::open(redis_host).expect("cannot open redis client");
    let redis = redis_client
      .get_connection()
      .expect("cannot connect to redis server");

    let redis = Arc::new(Mutex::new(redis));

    Model {
      job: Job::new(redis),
    }
  }

  pub fn job_model(self) -> Job {
    self.job
  }
}
