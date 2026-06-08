use serde::{Deserialize, Serialize};

use crate::models::_entities::users;
use crate::models::ability::Ability;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub pid: String,
    pub name: String,
    pub roles: Vec<String>,
    pub can_manage_admin: bool,
    pub is_verified: bool,
    pub admin_sections: Vec<String>,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, roles: Vec<String>) -> Self {
        let ability = Ability::from_roles(roles.clone());
        let can_manage_admin = ability.can_manage_admin();
        let admin_sections = if can_manage_admin {
            ability.admin_sections().into_iter().map(String::from).collect()
        } else {
            Vec::new()
        };

        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            roles,
            can_manage_admin,
            is_verified: user.email_verified_at.is_some(),
            admin_sections,
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
    pub admin_sections: Vec<String>,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model, roles: Vec<String>) -> Self {
        let ability = Ability::from_roles(roles.clone());
        let can_manage_admin = ability.can_manage_admin();
        let admin_sections = if can_manage_admin {
            ability.admin_sections().into_iter().map(String::from).collect()
        } else {
            Vec::new()
        };

        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            roles,
            can_manage_admin,
            admin_sections,
        }
    }
}
