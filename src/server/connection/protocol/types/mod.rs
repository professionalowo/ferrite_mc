pub mod serialize;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct VarInt(pub i32);

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct VarLong(pub i64);

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct UUID(pub u128);

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct MString {
    pub size: VarInt,
    pub value: String,
}
