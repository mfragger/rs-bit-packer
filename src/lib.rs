use std::{collections::HashMap, default};

#[derive(PartialEq)]
pub struct BitPack {
    bytes : Vec<u8>,
    bit_info: Vec<BitInfo>,
    bit_find: HashMap<String, usize>,
    last_byte_size_remaining : u8,
}

#[derive(PartialEq, Clone, Debug, Default)]
struct BitInfo {
    starting_bit_mask: u8,
    ending_bit_mask: u8,
    starting_array_index_inc: usize,
    ending_array_index_exc: usize,
}

pub trait Construct {
    fn new() -> Self;
}

impl Construct for BitInfo {
    fn new() -> Self {
        BitInfo {
            ending_array_index_exc: 0,
            ending_bit_mask: 0,
            starting_array_index_inc: 0,
            starting_bit_mask: 0,
        }
    }
}

impl Construct for BitPack {
    fn new () -> Self {
        BitPack {
            bytes: vec![0],
            bit_info: Vec::new(),
            bit_find: HashMap::new(),
            last_byte_size_remaining: 8,
        }
    }
}

impl BitPack {
    pub fn set(&mut self, name: &str, to_alloc_size: u8, initial_value: u8) {

        if to_alloc_size == 0{
            println!("to_alloc cannot be used");
            return;
        }
        
        let can_fit : bool = self.last_byte_size_remaining >= to_alloc_size;
        let mut result: usize = 0;
        let starting_index: usize = self.bytes.len() - 1;

        if !(can_fit) {
            println!("make new space!!! but how many??");

            result = (to_alloc_size as f32 / 8f32).ceil() as usize;
            self.bytes.extend(vec![0; result]);
            self.last_byte_size_remaining += 8 * result as u8;
        }

        let last_bit_info  =  if let Some(last) = self.bit_info.last() {
            last
        } else {
            &BitInfo {
                starting_bit_mask: 0,
                ending_bit_mask: 0,
                starting_array_index_inc: 0,
                ending_array_index_exc: 0,
            }
        };

        let shift_with = last_bit_info.ending_bit_mask.trailing_ones();
        let mut binding : BitInfo = Default::default();
        let limit = result + 1;

        for i in 0..limit {
            // println!("when i is {}, and shift_with is: {}", i, shift_with);
            let shifted_value : u8;
            //now I shift
            match  i {
                0 => {
                    shifted_value = initial_value << shift_with;
                }
                _ => {
                    shifted_value = initial_value >> shift_with * i as u32;
                }
            }

            //OR MASK
            self.bytes[ if last_bit_info.ending_array_index_exc <= 0 {
                0
            } else {
                last_bit_info.ending_array_index_exc - 1
            }] |= shifted_value;

            //Store info
            if i == 0 {
                binding.starting_bit_mask = Self::get_mask(to_alloc_size) << shift_with;
                binding.starting_array_index_inc = starting_index;
            }

            if i == result {
                binding.ending_bit_mask = if limit == 1 {
                    binding.starting_bit_mask
                } else {
                    Self::get_mask(to_alloc_size) >> shift_with * i as u32
                };
                binding.ending_array_index_exc = self.bytes.len();
            }
        }

        self.bit_info.push(binding);
        self.bit_find.insert(name.to_owned(), self.bit_info.len() - 1);
        self.last_byte_size_remaining -= to_alloc_size;
        
    }

    fn get_bit_info (&self, index : usize) -> &BitInfo {
        &self.bit_info[index]
    }
    
    fn get_mask (alloc_size: u8) -> u8 {
        match alloc_size {
            1 => 0b0000_0001,
            2 => 0b0000_0011,
            3 => 0b0000_0111,
            4 => 0b0000_1111,
            5 => 0b0001_1111,
            6 => 0b0011_1111,
            7 => 0b0111_1111,
            8 => 0b1111_1111,
            _ => 0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::BitInfo;
    use crate::Construct;
    use crate::BitPack;
    
    #[test]
    fn insert_data_one_data() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });
    }

    #[test]
    fn insert_data_two_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b1111_0000,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });
    }

    #[test]
    fn insert_data_two_datas_with_overflow() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set("World", 5, 30);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b0000_0001,
            starting_array_index_inc: 0,
            ending_array_index_exc: 2
        });
    }

    // #[test]
    // fn insert_data_three_datas() {
    //     let mut bit_pack : BitPack = BitPack::new();
    //     bit_pack.set("Hello", 4, 7);
        
    //     assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
    //         ending_bit_mask
    //         starting_array_index_inc: 0,
    //         ending_array_index_exc: 1
    //     });

    //     bit_pack.set("World", 4, 6);

    //     assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
    //         starting_array_index_inc: 0,
    //         ending_array_index_exc: 1
    //     });

    //     bit_pack.set("Zack", 4, 3);

    //     assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
    //         starting_array_index_inc: 1,
    //         ending_array_index_exc: 2
    //     });
    // }

    // #[test]
    // fn insert_data_four_datas() {
    //     let mut bit_pack : BitPack = BitPack::new();
    //     bit_pack.set("Hello", 4, 7);
        
    //     assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            
    //         starting_array_index_inc: 0,
    //         ending_array_index_exc: 1
    //     });

    //     bit_pack.set("World", 4, 6);

    //     assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
    //         starting_array_index_inc: 0,
    //         ending_array_index_exc: 1
    //     });

    //     bit_pack.set("Zack", 4, 3);

    //     assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
    //         starting_array_index_inc: 1,
    //         ending_array_index_exc: 2
    //     });

    //     bit_pack.set("Markd", 5, 30);

    //     assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
    //         starting_array_index_inc: 1,
    //         ending_array_index_exc: 3
    //     });
    // }
}