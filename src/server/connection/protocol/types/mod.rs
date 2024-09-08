#[derive(Debug, Clone)]
pub struct VarInt {
    pub value: i32,
    pub length: u8,
}
pub mod serialize;

#[derive(Debug, Clone)]
pub struct VarLong {
    pub value: i64,
    pub length: u8,
}
#[derive(Debug, Clone)]
pub struct UUID(pub u128);

#[derive(Debug, Clone)]
pub struct MString {
    pub size: VarInt,
    pub value: String,
}
