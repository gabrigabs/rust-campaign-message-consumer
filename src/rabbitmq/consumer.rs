use crate::error::Result;
use crate::models::message::{Message, MessagePayload};
use crate::repositories::campaign_repository::CampaignRepository;
use crate::repositories::message_repository::MessageRepository;
use futures_lite::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use tracing::{info, error};

pub struct RabbitMQConsumer {
    connection: Connection,
    message_repository: MessageRepository,
    campaign_repository: CampaignRepository,
}

impl RabbitMQConsumer {
    pub async fn new(
        rabbitmq_url: &str,
        message_repository: MessageRepository,
        campaign_repository: CampaignRepository,
    ) -> Result<Self> {
        let connection = Connection::connect(
            rabbitmq_url,
            ConnectionProperties::default(),
        ).await?;

        Ok(Self {
            connection,
            message_repository,
            campaign_repository,
        })
    }

    pub async fn consume_messages(&self, queue_name: &str) -> Result<()> {
        let channel = self.connection.create_channel().await?;

        channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        ).await?;

        let mut consumer = channel.basic_consume(
            queue_name,
            "campaign-message-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await?;

        info!("Started consuming from queue: {}", queue_name);

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    let data = delivery.data.clone();
                    
                    match serde_json::from_slice::<MessagePayload>(&data) {
                        Ok(payload) => {
                            info!(
                                campaign_id = %payload.campaign_id,
                                company_id = %payload.company_id,
                                "Received message"
                            );

                            if let Err(e) = self.process_message(payload).await {
                                error!("Failed to process message: {}", e);
                            } 

                            delivery.ack(BasicAckOptions::default()).await?;
                        },
                        Err(e) => {
                            error!("Failed to deserialize message: {}", e);
                            delivery.ack(BasicAckOptions::default()).await?;
                        }
                    }
                },
                Err(e) => {
                    error!("Error receiving message: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn process_message(&self, payload: MessagePayload) -> Result<()> {
        let campaign_id = payload.campaign_id.clone();
        
        let message = Message::from_payload(payload);

        let message_id = self.message_repository.save_message(message).await?;
        info!("Saved message with ID: {}", message_id);

        self.campaign_repository.update_campaign_status(&campaign_id, "SENT").await?;

        Ok(())
    }
}