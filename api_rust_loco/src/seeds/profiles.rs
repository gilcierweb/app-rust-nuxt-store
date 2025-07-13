use loco_rs::Result;
use chrono::{DateTime, TimeZone, Utc};
use sea_orm::{ActiveModelTrait, Set, EntityTrait, PaginatorTrait};
use fakeit::{internet, name, contact, image, words, datetime};

use crate::models::_entities::profiles::{ActiveModel as Profile, Entity as Profiles};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Profiles::find().count(db).await?;
    if count > 0 {
        tracing::info!("Profiles já existem, ignorando.");
        return Ok(());
    }

    for i in 0..10 {
        let birth_dt = {
            let data = datetime::date();
        
            // Usa DateTime<Utc>::from_timestamp, que é a versão correta e suportada
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
            user_id: Set(1),
            ..Default::default()
        };

        profile.insert(db).await?;
    }

    tracing::info!("10 perfis gerados com fakeit ✅");
    Ok(())
}