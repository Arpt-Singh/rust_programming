#![allow(dead_code, unused_variables)]


mod database;

pub mod auth_utils;

use database::{connect_to_database,Status};

use auth_utils::login;

pub fn authentication(cred: auth_utils::models::Credentials) {

    if let Status::Connected = connect_to_database() {

        login(cred);

    }
}