use crate::error::Result;
use crate::models::message::Message;
use mongodb::{Collection, Database};

pub struct MessageRepository {
    collection: Collection<Message>,
}

impl MessageRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<Message>("messages");
        Self { collection }
    }

    pub async fn save_message(&self, message: Message) -> Result<String> {
        let message_id = message.id.clone().unwrap_or_default();
        self.collection.insert_one(message, None).await?;
        Ok(message_id)
    }
}