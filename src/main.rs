#![feature(proc_macro_hygiene, decl_macro, result_flattening)]
extern crate rocket;
use rocket::config::{Config, Environment};
use rocket::routes;

mod auth;
pub mod bigquery;
mod crawl;
mod map_json;
mod scan;
mod status;
mod up;
mod util;

/*


*/

pub fn get_env(name: &'static str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("missing environment variable: {}", name))
}

fn main() {
    let Config = Config::build(Environment::Staging).finalize();

    rocket::custom(config)
        .mount("/", routes![up::catch_up])
        .mount("/", routes![scan::catch_scan])
        .mount("/", routes![crawl::catch_crawl])
        .mount("/", routes![status::catch_ready])
        .mount("/", routes![status::catch_health])
        .launch();
}
