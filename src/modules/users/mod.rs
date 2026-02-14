pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

pub use controller::routes;

#[cfg(test)]
mod tests;
