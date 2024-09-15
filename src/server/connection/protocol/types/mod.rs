pub mod serialize;

fn get_var_length(val: i64) -> u32 {
    if val == 0 {
        return 1; // Zero requires 1 byte
    }

    // Count the number of bits needed to represent the value
    let mut bits = 0;
    let mut temp_value = val;

    while temp_value != 0 {
        bits += 1;
        temp_value >>= 1;
    }

    // Calculate the number of bytes required (each byte encodes 7 bits)
    (bits + 6) / 7 // Adding 6 ensures we round up when dividing by 7
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct VarInt(pub i32);

impl VarInt {
    pub fn length(&self) -> u32 {
        get_var_length(self.0 as i64)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct VarLong(pub i64);

impl VarLong {
    pub fn length(&self) -> u32 {
        get_var_length(self.0)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct UUID(pub u128);

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct MString {
    pub size: VarInt,
    pub value: String,
}
