use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use cuid2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub phone_number: String,
    pub message: String,
    pub campaign_id: String,
    pub company_id: String,
    #[serde(with = "bson_datetime_format", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "bson_datetime_format", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    pub phone_number: String,
    pub message: String,
    pub campaign_id: String,
    pub company_id: String,
}

impl Message {
    pub fn from_payload(payload: MessagePayload) -> Self {
        let now = Utc::now();
        
        Self {
            id: Some(cuid2::create_id()),
            phone_number: payload.phone_number,
            message: payload.message,
            campaign_id: payload.campaign_id,
            company_id: payload.company_id,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

mod bson_datetime_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        date: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => serializer.serialize_i64(date.timestamp_millis()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<i64>::deserialize(deserializer)?;
        
        match opt {
            Some(timestamp) => Ok(Some(Utc.timestamp_millis_opt(timestamp).single().unwrap())),
            None => Ok(None),
        }
    }
}