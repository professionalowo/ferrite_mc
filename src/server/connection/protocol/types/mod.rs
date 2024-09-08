pub mod serialize;

#[derive(Debug, Clone)]
pub struct VarInt(pub i32);

#[derive(Debug, Clone)]
pub struct VarLong(pub i64);

#[derive(Debug, Clone)]
pub struct UUID(pub u128);

#[derive(Debug, Clone)]
pub struct MString {
    pub size: VarInt,
    pub value: String,
}
