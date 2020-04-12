#[macro_use]
extern crate log;
extern crate redis;

mod job;
mod redis_instance;

// TODO: make this as a web service
#[tokio::main]
async fn main() {
    env_logger::init();

    let redis_instance = redis_instance::connect_redis();

    // for example
    let mut example_job = job::Job::new(
        redis_instance, // TODO: handle better way for this
        String::from("p_ke4guri8dh2pgpx"),
        String::from("d_fj2icskbtuo9odz"),
        String::from("master"),
        String::from("https://github.com/evilfactorylabs/alchemy.git"),
        String::from("'npm run build && npm run export'"),
        String::from("npm"),
        String::from("out"),
    );

    example_job.run().await;
}
