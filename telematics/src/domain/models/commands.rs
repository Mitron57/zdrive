use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: Uuid,
    pub car_id: Uuid,
    pub command_type: CommandType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    OpenDoor,
    CloseDoor,
    LockDoor,
    UnlockDoor,
    LockEngine,
    UnlockEngine,
    StartEngine,
    StopEngine,
}

impl CommandType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::OpenDoor => "open_door",
            CommandType::CloseDoor => "close_door",
            CommandType::LockDoor => "lock_door",
            CommandType::UnlockDoor => "unlock_door",
            CommandType::LockEngine => "lock_engine",
            CommandType::UnlockEngine => "unlock_engine",
            CommandType::StartEngine => "start_engine",
            CommandType::StopEngine => "stop_engine",
    }
    }
}

impl std::str::FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open_door" => Ok(CommandType::OpenDoor),
            "close_door" => Ok(CommandType::CloseDoor),
            "lock_door" => Ok(CommandType::LockDoor),
            "unlock_door" => Ok(CommandType::UnlockDoor),
            "lock_engine" => Ok(CommandType::LockEngine),
            "unlock_engine" => Ok(CommandType::UnlockEngine),
            "start_engine" => Ok(CommandType::StartEngine),
            "stop_engine" => Ok(CommandType::StopEngine),
            _ => Err(format!("Invalid command type: {}", s)),
        }
    }
}

#[derive(Deserialize)]
pub struct SendCommandRequest {
    pub car_id: Uuid,
    pub command_type: String,
}

