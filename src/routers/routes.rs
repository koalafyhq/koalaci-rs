use crate::models::Model;
use crate::routers::job_wrapper;

use warp::{Filter, Rejection, Reply};

pub fn new(model: Model) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  let job_model = model.job_model();

  job_wrapper(job_model)
}
