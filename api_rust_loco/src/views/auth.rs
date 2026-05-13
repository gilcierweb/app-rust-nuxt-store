use serde::{Deserialize, Serialize};

use crate::models::_entities::users;
use crate::models::ability::Ability;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub roles: Vec<String>,
    pub can_manage_admin: bool,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String, roles: Vec<String>) -> Self {
        let can_manage_admin = Ability::from_roles(roles.clone()).can_manage_admin();

        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            roles,
            can_manage_admin,
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub can_manage_admin: bool,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model, roles: Vec<String>) -> Self {
        let can_manage_admin = Ability::from_roles(roles.clone()).can_manage_admin();

        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            roles,
            can_manage_admin,
        }
    }
}
