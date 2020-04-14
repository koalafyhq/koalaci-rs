extern crate redis;

use crate::models::Db;

use redis::Commands;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Job {
  db: Db,
}

#[derive(Deserialize, Serialize)]
pub struct Deployment {
  pub deployment_id: String,
  pub project_id: String,
  pub project_branch: String,
  pub project_repo_url: String,
  pub project_build_command: String,
  pub project_package_manager: String,
  pub project_dist_directory: String,
}

impl Job {
  pub fn new(db: Db) -> Job {
    Job { db: db }
  }

  pub fn is_current_job_running(&self, user_id: &String, job_id: &String) -> bool {
    let job = format!("deployment:{}:{}", user_id, job_id);

    let mut db = self.db.lock().unwrap();
    let result: String = db.get(job).unwrap_or("".to_string());

    result == "BUILDING"
  }

  pub fn update_job_status(&self, user_id: &String, deployment_id: &String, status: String) {
    let job = format!("deployment:{}:{}", user_id, deployment_id);

    let mut db = self.db.lock().unwrap();
    let _: () = db.set(job, status).unwrap();
  }

  pub fn clear_log_if_exists(&self, user_id: &String, job_id: &String) {
    let log = format!("log:{}:{}", user_id, job_id);

    let mut db = self.db.lock().unwrap();
    let check_existing_log: String = db.get(&log).unwrap_or("".to_string());

    if check_existing_log != "" {
      let _: () = db.del(log).unwrap();
    }
  }

  pub fn write_log(&self, user_id: &String, deployment_id: &String, line: String) {
    let log = format!("log:{}:{}", user_id, deployment_id);
    let line = format!("{} \n", line);

    let mut db = self.db.lock().unwrap();
    let _: () = db.append(log, line).unwrap();
  }

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
