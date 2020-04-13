extern crate redis;

use crate::models::Db;
use redis::Commands;

#[derive(Clone)]
pub struct Job {
  db: Db,
}

impl Job {
  pub fn new(db: Db) -> Job {
    Job { db: db }
  }

  // TODO: create this
  pub fn create(&self, user_id: String) -> String {
    let job = format!("deployment:{}", user_id);

    let mut db = self.db.lock().unwrap();
    let result: String = db.get(job).unwrap_or("".to_string());

    result
  }

  // TODO: Can we store `user_id` on context?
  pub fn status(&self, user_id: String, job_id: String) -> String {
    let job = format!("deployment:{}:{}", user_id, job_id);

    let mut db = self.db.lock().unwrap();
    let result: String = db.get(job).unwrap_or("".to_string());

    result
  }

  pub fn log(&self, user_id: String, job_id: String) -> String {
    let log = format!("log:{}:{}", user_id, job_id);

    let mut db = self.db.lock().unwrap();
    let result: String = db.get(log).unwrap_or("".to_string());

    result
  }
}
