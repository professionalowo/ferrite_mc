#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionState {
    Uninitialized = 0,
    Handshaking = 1,
    Login = 2,
    Configuration = 3,
    Play = 4,
}
