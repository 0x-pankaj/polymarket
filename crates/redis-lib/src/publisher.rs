use bb8_redis::redis::AsyncCommands;

use crate::pool::RedisPool;

pub struct RedisPublisher {
    pool: RedisPool,
}

impl RedisPublisher {
    pub fn new(pool: RedisPool) -> Self {
        RedisPublisher { pool }
    }

    pub async fn publish<T: serde::Serialize>(
        &self,
        channel: &str,
        message: &T,
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.get_conn().await?;
        let serialized = serde_json::to_string(message)?;
        let _: () = conn.publish(channel, serialized).await?;
        Ok(())
    }

    pub async fn push_to_queue<T: serde::Serialize>(
        &self,
        queue: &str,
        message: &T,
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.get_conn().await?;
        let serialized = serde_json::to_string(message)?;
        let _: () = conn.rpush(queue, serialized).await?;
        Ok(())
    }
}
