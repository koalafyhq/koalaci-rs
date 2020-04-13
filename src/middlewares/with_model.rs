use crate::models::Job;

use std::convert::Infallible;
use warp::Filter;

pub fn with_model(model: Job) -> impl Filter<Extract = (Job,), Error = Infallible> + Clone {
  warp::any().map(move || model.clone())
}
