use redis::Commands;
use std::process::Stdio;
use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command,
  stream::StreamExt,
};

pub struct Job {
  redis: redis::Connection,
  deployment_id: String,
  project_id: String,
  project_branch: String,
  project_repo_url: String,
  project_build_command: String,
  project_package_manager: String,
  project_dist_directory: String,
}

impl Job {
  pub fn new(
    redis: redis::Connection,
    id: String,
    deployment_id: String,
    branch: String,
    repo_url: String,
    build_command: String,
    package_manager: String,
    dist_directpry: String,
  ) -> Job {
    Job {
      redis,
      project_id: id,
      deployment_id: deployment_id,
      project_branch: branch,
      project_repo_url: repo_url,
      project_build_command: build_command,
      project_package_manager: package_manager,
      project_dist_directory: dist_directpry,
    }
  }

  pub async fn run(&mut self) {
    if self.is_job_running() {
      warn!("A job is currently in progress: {}", self.deployment_id);
      return;
    }

    self.run_ci().await;
  }

  fn get_active_job(&self) -> String {
    let job = format!("deployment:{}", self.deployment_id);

    job
  }

  fn set_build_status(&mut self, status: String) {
    let _: () = self.redis.set(self.get_active_job(), status).unwrap();
  }

  fn save_log_to_redis(&mut self, log: String) {
    let log = format!("{} \n", log);
    let log_key = format!("log:{}", self.deployment_id);
    let _: () = self.redis.append(log_key, log).unwrap();
  }

  fn set_env_vars(&self) -> String {
    let mut vars = String::from("");

    vars = format!("{} -e {}={}", vars, "DEPLOYMENT_ID", self.deployment_id);
    vars = format!("{} -e {}={}", vars, "PROJECT_ID", self.project_id);

    vars = format!(
      "{} -e {}={}",
      vars, "PROJECT_REPO_BRANCH", self.project_branch
    );

    vars = format!(
      "{} -e {}={}",
      vars, "PROJECT_REPO_URL", self.project_repo_url
    );

    vars = format!(
      "{} -e {}={}",
      vars, "PROJECT_PACKAGE_MANAGER", self.project_package_manager
    );

    vars = format!(
      "{} -e {}={}",
      vars, "PROJECT_DIST_DIRECTORY", self.project_dist_directory
    );

    vars
  }

  fn is_job_running(&mut self) -> bool {
    let result: String = self
      .redis
      .get(self.get_active_job())
      .unwrap_or("".to_string()); // if key is not exists, return empty string instead

    result != "" && result != "READY" && result != "ERROR"
  }

  async fn run_ci(&mut self) {
    self.set_build_status(String::from("BUILDING"));
    info!("Starting build project with id: {}", self.deployment_id);

    let args = format!(
      "ci/run.sh '{}' {}",
      self.set_env_vars().trim(),
      self.project_build_command.trim()
    );

    // TODO: how to capture the "trap" exit here?
    let mut cmd = Command::new("bash")
      .arg("-c")
      .arg(args)
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to execute process");

    let stdout = cmd.stdout.take().expect("OK");

    let _result: Vec<()> = BufReader::new(stdout)
      .lines()
      .map(|line| {
        let line = line.unwrap();
        info!("{}", line);
        self.save_log_to_redis(line);
      })
      .collect()
      .await;

    self.set_build_status(String::from("READY"));

    info!("Project with id {} is built and ready", self.deployment_id);
  }
}
