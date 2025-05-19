use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;

pub struct RedisPool {
    pool: Pool<RedisConnectionManager>,
}

impl RedisPool {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let pool = Pool::builder().max_size(100).build(manager).await?;
        Ok(RedisPool { pool })
    }

    pub async fn get_conn(&self) -> anyhow::Result<PooledConnection<RedisConnectionManager>> {
        Ok(self.pool.get().await?)
    }
}
