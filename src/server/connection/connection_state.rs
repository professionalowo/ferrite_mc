use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionState {
    Uninitialized = 0,
    Handshaking = 1,
    Login = 2,
    Configuration = 3,
    Play = 4,
}
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    version: Version,
    players: Players,
    description: Description,
    favicon: String,
    enforcesSecureChat: bool,
}
#[derive(Debug, Serialize)]
struct Version {
    name: String,
    protocol: u32,
}
#[derive(Debug, Serialize)]
struct Players {
    max: u16,
    online: u16,
    sample: Vec<Player>,
}
#[derive(Debug, Serialize)]
struct Player {
    name: String,
    id: String,
}
#[derive(Debug, Serialize)]
struct Description {
    text: String,
}

pub fn test_response() -> StatusResponse {
    StatusResponse {
        favicon: "".into(),
        description: Description {
            text: "RUST".into(),
        },
        enforcesSecureChat: false,
        players: Players {
            max: 1,
            online: 0,
            sample: Vec::new(),
        },
        version: Version {
            name: "1.20.1".into(),
            protocol: 767,
        },
    }
}
