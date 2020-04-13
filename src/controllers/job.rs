use crate::models::Job;
use std::convert::Infallible;
use warp::Reply;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload {
  id: String,
}

// TODO: finish this
pub async fn create_job(
  payload: Payload,
  user_id: String,
  job: Job,
) -> Result<impl Reply, Infallible> {
  let _result = job.create(user_id);

  Ok(warp::reply::json(&payload))
}

pub async fn get_job_status(
  job_id: String,
  user_id: String,
  job: Job,
) -> Result<impl Reply, Infallible> {
  let result = job.status(user_id, job_id);

  Ok(warp::reply::html(format!("Job id {}", result)))
}

pub async fn get_log(job_id: String, user_id: String, job: Job) -> Result<impl Reply, Infallible> {
  let result = job.log(user_id, job_id);

  Ok(warp::reply::html(format!("<pre>{}</pre>", result)))
}
