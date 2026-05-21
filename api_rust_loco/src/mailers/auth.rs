// auth mailer
#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;

use crate::models::users;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static magic_link: Dir<'_> = include_dir!("src/mailers/auth/magic_link");
// #[derive(Mailer)] // -- disabled for faster build speed. it works. but lets
// move on for now.

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &forgot,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sends a magic link authentication email to the user.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_magic_link(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &magic_link,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model not contains magic link token",
                    ))?,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn mail_template_public(
        ctx: &AppContext,
        template_name: &str,
        to: String,
        locals: serde_json::Value,
    ) -> Result<()> {
        let dir = match template_name {
            "welcome" => &welcome,
            "forgot_password" | "forgot" => &forgot,
            "magic_link" => &magic_link,
            _ => return Err(Error::string(&format!("Unknown template: {template_name}"))),
        };
        Self::mail_template(
            ctx,
            dir,
            mailer::Args {
                to,
                locals,
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }

    pub fn get_all_templates() -> Vec<TemplateInfo> {
        let mut list = Vec::new();
        let templates = vec![
            ("welcome", &welcome),
            ("forgot_password", &forgot),
            ("magic_link", &magic_link),
        ];

        for (name, dir) in templates {
            let subject = dir
                .get_file("subject.txt")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .trim()
                .to_string();

            let html = dir
                .get_file("html.tera")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .to_string();

            let text = dir
                .get_file("text.tera")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .to_string();

            list.push(TemplateInfo {
                name: name.to_string(),
                subject,
                html,
                text,
            });
        }

        list
    }
}

#[derive(serde::Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub subject: String,
    pub html: String,
    pub text: String,
}
