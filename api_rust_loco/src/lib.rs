#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "pt-BR");

pub mod app;
pub mod controllers;
pub mod data;
pub mod initializers;
pub mod mailers;
pub mod models;
pub mod tasks;
pub mod views;
pub mod workers;
pub mod seeds;
pub mod openapi;
pub mod utils;
