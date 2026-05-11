use loco_rs::Result;
use chrono::{DateTime, TimeZone, Utc};
use sea_orm::{ActiveModelTrait, Set, EntityTrait, PaginatorTrait};
use fakeit::{internet, name, contact, image, words, datetime};

use crate::models::_entities::profiles::{ActiveModel as Profile, Entity as Profiles};
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Profiles::find().count(db).await?;
    if count > 0 {
        tracing::info!("Profiles already exist, skipping.");
        return Ok(());
    }

    let users = UserEntity::find().all(db).await?;
    if users.is_empty() {
        tracing::warn!("No users found. Profiles cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for (i, user) in users.iter().enumerate() {
        let birth_dt = {
            let data = datetime::date();
            let datetime = DateTime::<Utc>::from_timestamp(data.secs, data.nsecs)
                .unwrap_or_else(|| Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap());
            datetime.naive_utc().date()
        };

        let first = name::first();
        let last = name::last();
        let username = format!("{}{}", first.to_lowercase(), i);
        let full = name::full();
        let nick = internet::username();
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let profile = Profile {
            first_name: Set(Some(first)),
            last_name: Set(Some(last)),
            full_name: Set(Some(full)),
            username: Set(Some(username)),
            nickname: Set(Some(nick)),
            phone: Set(Some(contact::phone().parse::<i64>().unwrap())),
            birth_date: Set(Some(birth_dt)),
            avatar: Set(Some(image::url(500, 500))),
            bio: Set(Some(paragraph)),
            whatsapp: Set(Some(contact::phone().parse::<i64>().unwrap())),
            user_id: Set(user.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        profile.insert(db).await?;
    }

    tracing::info!("{} profiles generated with fakeit", users.len());
    Ok(())
}