use crate::models::job::Deployment;
use crate::models::Job;

use serde::{Deserialize, Serialize};
use std::process::Stdio;

use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command,
};

#[derive(Serialize, Deserialize)]
struct Response {
  data: String,
}

use std::convert::Infallible;
use warp::Reply;

pub async fn create_job(
  payload: Deployment,
  user_id: String,
  job: Job,
) -> Result<impl Reply, Infallible> {
  let is_job_running = job.is_current_job_running(&user_id, &payload.deployment_id);

  if is_job_running {
    return Ok(warp::reply::json(&Response {
      data: String::from("Current job is running"),
    }));
  }

  job.clear_log_if_exists(&user_id, &payload.deployment_id);
  job.update_job_status(&user_id, &payload.deployment_id, String::from("BUILDING"));

  let args = format!(
    "ci/run.sh '{}' '{}' {}",
    set_env_vars(&payload).trim(),
    payload.deployment_id,
    payload.project_build_command.trim()
  );

  let mut cmd = Command::new("bash")
    .arg("-c")
    .arg(args)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute process");

  let stdout = cmd.stdout.take().expect("echo-ing to stdout");

  let mut reader = BufReader::new(stdout).lines();

  tokio::spawn(async move {
    while let Some(line) = reader.next_line().await.unwrap() {
      debug!("{}", line);

      job.write_log(&user_id, &payload.deployment_id, line);
    }

    let status = cmd.await.expect("child process encountered an error");

    match status.code() {
      Some(137) => {
        job.update_job_status(&user_id, &payload.deployment_id, String::from("CANCELED"))
      }
      Some(0) => job.update_job_status(&user_id, &payload.deployment_id, String::from("READY")),
      _ => job.update_job_status(&user_id, &payload.deployment_id, String::from("ERROR")),
    }
  });

  Ok(warp::reply::json(&Response {
    data: String::from("OK"),
  }))
}

pub async fn cancel_job(
  job_id: String,
  user_id: String,
  job: Job,
) -> Result<impl Reply, Infallible> {
  let is_job_running = job.is_current_job_running(&user_id, &job_id);

  if is_job_running == false {
    return Ok(warp::reply::json(&Response {
      data: String::from("Nothing to cancel"),
    }));
  }

  let cmd = Command::new("bash")
    .arg("-c")
    .arg(format!("ci/stop.sh {}", job_id))
    .stdout(Stdio::inherit())
    .spawn()
    .expect("failed to execute process");

  tokio::spawn(async move {
    let status = cmd.await.expect("child process encountered an error");

    if !status.success() {
      warn!("Failed to cancel job for id {}", &job_id);
    }
  });

  Ok(warp::reply::json(&Response {
    data: String::from("Queued for cancellation"),
  }))
}

pub async fn get_job_status(
  job_id: String,
  user_id: String,
  job: Job,
) -> Result<impl Reply, Infallible> {
  let result = job.status(user_id, job_id);

  Ok(warp::reply::json(&Response { data: result }))
}

pub async fn get_log(job_id: String, user_id: String, job: Job) -> Result<impl Reply, Infallible> {
  let result = job.log(user_id, job_id);

  Ok(warp::reply::json(&Response { data: result }))
}

fn set_env_vars(payload: &Deployment) -> String {
  let mut vars = String::from("");

  vars = format!("{} -e {}={}", vars, "DEPLOYMENT_ID", payload.deployment_id);
  vars = format!("{} -e {}={}", vars, "PROJECT_ID", payload.project_id);

  vars = format!(
    "{} -e {}={}",
    vars, "PROJECT_REPO_BRANCH", payload.project_branch
  );

  vars = format!(
    "{} -e {}={}",
    vars, "PROJECT_REPO_URL", payload.project_repo_url
  );

  vars = format!(
    "{} -e {}={}",
    vars, "PROJECT_PACKAGE_MANAGER", payload.project_package_manager
  );

  vars = format!(
    "{} -e {}={}",
    vars, "PROJECT_DIST_DIRECTORY", payload.project_dist_directory
  );

  vars
}
