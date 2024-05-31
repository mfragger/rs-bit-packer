use std::collections::HashMap;

pub struct BitPack {
    bytes : Vec<u8>,
    bit_info: Vec<BitInfo>,
    bit_find: HashMap<String, u8>,
}

struct BitInfo {
    bit_type: BitType,
    starting_bit_number_inc: u8,
    ending_bit_number_exc: u8,
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
            ending_bit_number_exc: 0,
            starting_array_index_inc: 0,
            starting_bit_number_inc: 0,
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
    pub fn set(mut self, name: &str, size: u8, initial_value: u8) {
        
        let size_type: BitType = match size {
            1..=8 => BitType::U8(0),
            9..=16 => BitType::U16(0),
            17..=32 => BitType::U32(0),
            33..=64 => BitType::U16(0),
            65..=128 => BitType::U128(0),
            _ => panic!("Value out of range"),
        };

        let value_type: BitType = match 128 - initial_value.leading_zeros() {
            1..=8 => BitType::U8(0),
            9..=16 => BitType::U16(0),
            17..=32 => BitType::U32(0),
            33..=64 => BitType::U16(0),
            65..=128 => BitType::U128(0),
            _ => panic!("Value out of range"),
        };

        //check space for size (mask_type)
        let last_element = self.bit_info.last().unwrap_or(&BitInfo {
            bit_type : BitType::U8(0),
            starting_bit_number_inc : 0,
            ending_array_index_exc : 0,
            ending_bit_number_exc : 0,
            starting_array_index_inc : 0,
        });

        if last_element.starting_bit_number_inc == last_element.ending_bit_number_exc {
            let mut new_bit_info : BitInfo = BitInfo::new();
            new_bit_info.starting_bit_number_inc = last_element.ending_bit_number_exc;
            new_bit_info.ending_bit_number_exc = last_element.ending_bit_number_exc + size + 1;
            new_bit_info.starting_array_index_inc = last_element.ending_array_index_exc;
            new_bit_info.ending_array_index_exc = last_element.ending_bit_number_exc + 1;
            self.bit_info.push(new_bit_info);
        }

        self.bytes.push(self.bytes.last().unwrap_or(&0) | initial_value as u8);
        self.bit_find.insert(name.to_owned(), (self.bit_info.len() - 1).try_into().unwrap());
    }
}
