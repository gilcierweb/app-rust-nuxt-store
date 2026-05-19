#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "pt-BR");

pub mod app;
pub mod cache;
pub mod controllers;
pub mod data;
pub mod initializers;
pub mod mailers;
pub mod middleware;
pub mod models;
pub mod openapi;
pub mod payment_gateways;
pub mod seeds;
pub mod tasks;
pub mod utils;
pub mod views;
pub mod workers;
