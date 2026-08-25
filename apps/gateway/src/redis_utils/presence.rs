use std::str::FromStr;

use redis::{AsyncCommands, aio::MultiplexedConnection};

#[derive(PartialEq, Clone)]
pub enum PresenceStatus {
    Online,
    Offline,
    Idle,
    DoNotDisturb,
}

impl PresenceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Idle => "idle",
            Self::DoNotDisturb => "dnd",
        }
    }
}

impl FromStr for PresenceStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "online" => Ok(Self::Online),
            "offline" => Ok(Self::Offline),
            "idle" => Ok(Self::Idle),
            "dnd" | "donotdisturb" => Ok(Self::DoNotDisturb),
            _ => Err(()),
        }
    }
}
pub async fn set_user_status(
    redis: &mut MultiplexedConnection,
    user_id: &i32,
    channel_id: &i32,
    status: PresenceStatus,
) -> Result<(), redis::RedisError> {
    let key = format!("presence:user:{user_id}");
    let status_str: &str = PresenceStatus::as_str(&status);
    if status != PresenceStatus::Offline {
        redis.del::<_, ()>(&key).await?;
    } else {
        redis.set_ex::<_, _, ()>(&key, &status_str, 45).await?;
    }

    let event_payload = serde_json::json!({
        "event": "USER_PRESENCE_CHANGED",
        "payload": {
            "user_id": user_id,
            "status": status_str
        }
    })
    .to_string();

    redis
        .publish(format!("channel:{channel_id}:events"), event_payload)
        .await
}

pub fn get_presence_key(user_id: &i32) -> String {
    return format!("presence:user:{user_id}");
}
