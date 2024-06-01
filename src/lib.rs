use core::num;
use std::{collections::HashMap, ops::BitOr};

#[derive(PartialEq)]
pub struct BitPack {
    bytes : Vec<u8>,
    bit_info: Vec<BitInfo>,
    bit_find: HashMap<String, u8>,
}

#[derive(PartialEq, Clone, Debug)]
struct BitInfo {
    bit_type: BitType,
    starting_bit_position_inc: u8,
    ending_bit_position_exc: u8,
    starting_array_index_inc: u8,
    ending_array_index_exc: u8,
}

#[derive(PartialEq, Clone, Debug)]
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
    pub fn set(&mut self, name: &str, to_alloc_size: u8, initial_value: u8) {
        
        // let size_type: BitType = match size {
        //     1..=8 => BitType::U8(0),
        //     9..=16 => BitType::U16(0),
        //     17..=32 => BitType::U32(0),
        //     33..=64 => BitType::U16(0),
        //     65..=128 => BitType::U128(0),
        //     _ => panic!("Value out of range"),
        // };

        let binding = BitInfo::new();
        let last_bit_info: BitInfo = self.bit_info.last().unwrap_or(&binding).to_owned();

        self.bit_info.push(Self::new_bit_info(&last_bit_info, to_alloc_size, self.bytes.len()));

        if (to_alloc_size + last_bit_info.starting_bit_position_inc) > 8 || last_bit_info.starting_bit_position_inc == last_bit_info.ending_bit_position_exc {
            self.bytes.push(initial_value);
        } else {
            let last_index = self.bytes.len() - 1;
            let _last_index_u8 = last_index as u8;
            for i in (_last_index_u8)..last_bit_info.ending_array_index_exc {
                let new_value: u8;
                match i {
                    last_index => {
                        new_value = initial_value << to_alloc_size;
                        self.bytes[last_index as usize] = self.bytes[last_index as usize] | new_value;
                    }
                    _ => {
                        new_value = initial_value >> to_alloc_size * i;
                        self.bytes.push(new_value);
                    }
                }
            }
        }
        
        self.bit_find.insert(name.to_owned(), (self.bit_info.len() - 1).try_into().unwrap());
    }

    pub fn get_bit_info (&self, index : usize) -> &BitInfo {
        &self.bit_info[index]
    }

    fn new_bit_info (last_bit_info : &BitInfo, to_alloc_size: u8, array_size: usize) -> BitInfo {

        let is_first_run = array_size == 0;

        let mut num_allocate = 0;
        let mut space_left = 0;
        if last_bit_info.ending_bit_position_exc != 0 {
            space_left = (last_bit_info.ending_bit_position_exc.abs_diff(1)).checked_sub(8).unwrap_or(to_alloc_size + 1);
        }

        if space_left < to_alloc_size {
            num_allocate = ((to_alloc_size - space_left) + 7) /8;
        } else if !is_first_run && last_bit_info.ending_array_index_exc == 0 {
            num_allocate = 1;

        }

        let new_ending_bit_postion = last_bit_info.ending_bit_position_exc + to_alloc_size; 
        
        let bit_info = BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: last_bit_info.ending_bit_position_exc,
            ending_bit_position_exc: if new_ending_bit_postion > 8 { 
                    0 
                } else { 
                    new_ending_bit_postion 
                }
                + if last_bit_info.ending_bit_position_exc == 0 {
                    1
                } else {
                    0
                },
            starting_array_index_inc: if is_first_run {
                0
            } else {
                if num_allocate == 0 {
                    last_bit_info.starting_array_index_inc
                } else {
                    last_bit_info.starting_array_index_inc + 1
                }
            },
            ending_array_index_exc: if is_first_run {
                1
            } else {
                last_bit_info.ending_array_index_exc + num_allocate
            },
        };

        bit_info

    }
    
}

#[cfg(test)]
mod test {
    use crate::BitInfo;
    use crate::Construct;
    use crate::BitPack;
    use crate::BitType;
    
    #[test]
    fn insert_data_one_data() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });
    }

    #[test]
    fn insert_data_two_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 5,
            ending_bit_position_exc: 0,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });
    }

    #[test]
    fn insert_data_three_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 5,
            ending_bit_position_exc: 0,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("Zack", 4, 3);

        assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 1,
            ending_array_index_exc: 2
        });
    }

    #[test]
    fn insert_data_four_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 5,
            ending_bit_position_exc: 0,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("Zack", 4, 3);

        assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 0,
            ending_bit_position_exc: 5,
            starting_array_index_inc: 1,
            ending_array_index_exc: 2
        });

        bit_pack.set("Markd", 5, 30);

        assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
            bit_type: BitType::U8(0),
            starting_bit_position_inc: 5,
            ending_bit_position_exc: 1,
            starting_array_index_inc: 1,
            ending_array_index_exc: 3
        });
    }

}