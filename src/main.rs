mod config;
mod infrastructures;
mod models;
mod route;
mod services;
use crate::route::Router;

fn main() {
    Router::route();
}
