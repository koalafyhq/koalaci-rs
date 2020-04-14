use crate::controllers;
use crate::middlewares;
use crate::models::Job;

use warp::{Filter, Rejection, Reply};

// /job, a wrapper
pub fn job_wrapper(job: Job) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  status(job.clone())
    .or(log(job.clone()))
    .or(create(job.clone()))
    .or(cancel(job.clone()))
}

// POST — /job/create, create a job
fn create(job: Job) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("job" / "create")
    .and(warp::post())
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::json())
    .and(middlewares::with_auth())
    .and(middlewares::with_model(job))
    .and_then(controllers::job::create_job)
}

// PATH — /job/:id/cancel, cancel the job
fn cancel(job: Job) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path("job")
    .and(warp::patch())
    .and(warp::path::param())
    .and(warp::path("cancel"))
    .and(warp::path::end())
    .and(middlewares::with_auth())
    .and(middlewares::with_model(job))
    .and_then(controllers::job::cancel_job)
}

// GET — /job/:id/status, get job status
fn status(job: Job) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path("job")
    .and(warp::get())
    .and(warp::path::param())
    .and(warp::path("status"))
    .and(warp::path::end())
    .and(middlewares::with_auth())
    .and(middlewares::with_model(job))
    .and_then(controllers::job::get_job_status)
}

// GET — /job/:id/log, get job log
fn log(job: Job) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path("job")
    .and(warp::get())
    .and(warp::path::param())
    .and(warp::path("log"))
    .and(warp::path::end())
    .and(middlewares::with_auth())
    .and(middlewares::with_model(job))
    .and_then(controllers::job::get_log)
}
