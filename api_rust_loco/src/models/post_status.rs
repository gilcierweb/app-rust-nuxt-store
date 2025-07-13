use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(i32)]
pub enum PostStatus {
    Draft = 1,
    PendingReview = 2,
    Scheduled = 3,
    Published = 4,
    Private = 5,
    Archived = 6,
    Trashed = 7,
    Rejected = 8,
}

impl TryFrom<i32> for PostStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PostStatus::Draft),
            2 => Ok(PostStatus::PendingReview),
            3 => Ok(PostStatus::Scheduled),
            4 => Ok(PostStatus::Published),
            5 => Ok(PostStatus::Private),
            6 => Ok(PostStatus::Archived),
            7 => Ok(PostStatus::Trashed),
            8 => Ok(PostStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl From<PostStatus> for i32 {
    fn from(status: PostStatus) -> Self {
        status as i32
    }
}

impl<'de> Deserialize<'de> for PostStatus {
    fn deserialize<D>(deserializer: D) -> Result<PostStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        PostStatus::try_from(value)
            .map_err(|_| DeError::custom(format!("Invalid value for PostStatus: {}", value)))
    }
}
