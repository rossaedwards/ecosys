use crate::prelude::*;
use redis::Client as RedisClient;

pub struct Redis {
    client: RedisClient,
}

impl Redis {
    pub async fn connect(url: &str) -> Result<Self> {
        let client = RedisClient::open(url)
            .map_err(|e| Error::Redis(e))?;
        
        Ok(Self { client })
    }

    pub fn client(&self) -> &RedisClient {
        &self.client
    }
}

