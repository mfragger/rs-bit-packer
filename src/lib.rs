use std::collections::HashMap;

pub struct BitPack {
    bytes : Vec<u8>,
    bit_info: Vec<BitInfo>,
    bit_find: HashMap<String, u8>,
}

struct BitInfo {
    bit_type: BitType,
    starting_bit_position_inc: u8,
    ending_bit_position_exc: u8,
    starting_array_index_inc: u8,
    ending_array_index_exc: u8,
}

#[derive(Debug)]
pub enum BitType  {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub trait Construct {
    fn new() -> Self;
}

impl Construct for BitInfo {
    fn new() -> Self {
        BitInfo {
            bit_type: BitType::U8(0),
            ending_array_index_exc: 0,
            ending_bit_position_exc: 0,
            starting_array_index_inc: 0,
            starting_bit_position_inc: 0,
        }
    }
}

impl Construct for BitPack {
    fn new () -> Self {
        BitPack {
            bytes: Vec::new(),
            bit_info: Vec::new(),
            bit_find: HashMap::new(),
        }
    }
}

impl BitPack {
    pub fn set(&mut self, name: &str, size: u8, initial_value: u8) {
        
        let size_type: BitType = match size {
            1..=8 => BitType::U8(0),
            9..=16 => BitType::U16(0),
            17..=32 => BitType::U32(0),
            33..=64 => BitType::U16(0),
            65..=128 => BitType::U128(0),
            _ => panic!("Value out of range"),
        };

        //check space for size (mask_type)
        let last_bit_info = self.bit_info.last().unwrap_or(&BitInfo {
            bit_type : BitType::U8(0),
            starting_bit_position_inc : 0,
            ending_array_index_exc : 0,
            ending_bit_position_exc : 0,
            starting_array_index_inc : 0,
        });

        let mut new_bit_info : BitInfo = BitInfo::new();
        if last_bit_info.starting_bit_position_inc == last_bit_info.ending_bit_position_exc {
            //If we have no data
            new_bit_info.starting_bit_position_inc = last_bit_info.ending_bit_position_exc;
            new_bit_info.ending_bit_position_exc = last_bit_info.ending_bit_position_exc + size + 1;
            new_bit_info.starting_array_index_inc = last_bit_info.ending_array_index_exc;
            new_bit_info.ending_array_index_exc = last_bit_info.ending_array_index_exc + 1;

            self.bytes.push(self.bytes.last().unwrap_or(&0) | initial_value);

        } else {
            //if we have data
            //shift to left based on last info
            let shifted_value = initial_value << (last_bit_info.ending_bit_position_exc - 1);
            let new_value = self.bytes.last().unwrap_or(&0) | shifted_value;
             
            self.bytes[last_bit_info.starting_array_index_inc as usize] = new_value;
            
            new_bit_info.starting_bit_position_inc = last_bit_info.ending_bit_position_exc;
                
            let new_ending_bit_position_exc = last_bit_info.ending_bit_position_exc + size;

            new_bit_info.ending_bit_position_exc = if new_ending_bit_position_exc > 8 {
                0
            } else {
                new_ending_bit_position_exc
            };

            new_bit_info.starting_array_index_inc = 0;
            new_bit_info.ending_array_index_exc = 1;
        }
        self.bit_info.push(new_bit_info);
        self.bit_find.insert(name.to_owned(), (self.bit_info.len() - 1).try_into().unwrap());
    }
}
