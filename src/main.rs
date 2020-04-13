#[macro_use]
extern crate log;

mod controllers;
mod middlewares;
mod models;
mod routers;

use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let with_model = models::Model::new();
    let routers = routers::new(with_model);
    let app = routers.with(warp::log("koalaci"));

    warp::serve(app).run(([127, 0, 0, 1], 8081)).await;

    info!("Server run on http://localhost:8081");
}
