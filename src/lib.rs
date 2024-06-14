use std::{collections::HashMap, default};

#[derive(PartialEq)]
pub struct BitPack {
    bytes : Vec<u8>,
    bit_info: Vec<BitInfo>,
    bit_find: HashMap<String, usize>,
    last_byte_size_remaining : u8,
}

//0b0000_00[11|0b1111_1111]|0b[1110]_[0111]
//0b0000_00[11|0b1111_1111]
//0b1111_11[11|0b1111_1111]
//------------------------------
//0b0000_00[11|0b1111_11]00 >> 0b0000_0000|0b1111_1111 

//0b001_0010|0b1110_1111

//0b000_0000|0b000_0000|0b0000_[1111]

//target:
//0b001_0010_1110_1111 << 4

//0b000_0000|0b000_0000|0b1111_[1111]

//0b001_0010_1110_0000 >> 4 * 1

//0b000_0000|0b010_1110|0b1111_[1111]

//0b001_0010_1110_0000 >> 4 * 2

//0b000_000[1|0b0010_1110|0b1111_1111
//                       |0b0000_1111 << start and end bit mask
//
//                       |0b1111_0000
//                       ------------
// 0b000_0001|-----------|0b1111_0000

#[derive(PartialEq, Clone, Debug, Default)]
struct BitInfo {
    //starting_bit_pos
    //ending_bit_pos
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

trait FromSlice {
    fn from_slice(slice: &[u8]) -> Option<Self> where Self: Sized;
    // fn from_slice(slice: &[u8], num: u8) -> Option<Self> where Self: Sized;
}


impl FromSlice for u8 {
    fn from_slice(slice: &[u8]) -> Option<Self> {
        Some(slice[0])
    }
}

impl FromSlice for u16 {
    fn from_slice(slice: &[u8]) -> Option<Self> {
        let mut slice = slice.to_vec();
        if slice.len() == 1 {
            slice.push(0);
        }
        if slice.len() == 2 {
            Some(u16::from_le_bytes(slice.try_into().ok()?))
        } else {
            None
        }
    }
}

impl FromSlice for u32 {
    fn from_slice(slice: &[u8]) -> Option<Self> {
        let mut slice = slice.to_vec();
        
        while slice.len() < 4 {
            slice.push(0);
        }

        if slice.len() == 4 {
            Some(u32::from_le_bytes(slice.try_into().ok()?))
        } else {
            None
        }
    }
}

impl BitPack {
    pub fn set_new(&mut self, name: &str, to_alloc_size: u8, initial_value: u8) {

        if to_alloc_size == 0 {
            println!("to_alloc cannot be used");
            return;
        }
        
        let can_fit : bool = self.last_byte_size_remaining >= to_alloc_size;
        let mut result: usize = 0;
        let mut starting_index: usize = self.bytes.len() - 1;

        if !(can_fit) {
            println!("make new space!!! but how many??");
            result = (to_alloc_size as f32 / 8f32).ceil() as usize;
            self.bytes.extend(vec![0; result]);

            starting_index = if self.last_byte_size_remaining > 0 { 
                starting_index
            } else {
                self.bytes.len() - 1
            };

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
            self.bytes[i] |= shifted_value;

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
    
    pub fn get_u8(&self, arg: &str) -> u8 {
        let bit_info = &self.bit_info[self.bit_find[arg]];
        let slice = &self.bytes[bit_info.starting_array_index_inc..bit_info.ending_array_index_exc];
        let mut slice_mask = [1u8, slice.len() as u8];
        
        slice_mask[0] = bit_info.starting_bit_mask;
        slice_mask[slice_mask.len() - 1] = bit_info.ending_bit_mask;

        //thne from here do some bit shifting
        //then do an AND MASK
        let mut initial_value = u8::from_slice(slice).unwrap();
        initial_value = initial_value & u8::from_slice(&slice_mask).unwrap();
        if bit_info.starting_bit_mask.trailing_zeros() > 0
        {
            initial_value = initial_value >> bit_info.starting_bit_mask.trailing_zeros();
        }
        initial_value
    }

    pub fn get_u32(&self, arg: &str) -> u32 {
        let bit_info = &self.bit_info[self.bit_find[arg]];
        let slice = &self.bytes[bit_info.starting_array_index_inc..bit_info.ending_array_index_exc];
        let mut slice_mask = [1u8, slice.len() as u8];

        slice_mask[0] = bit_info.starting_bit_mask;
        slice_mask[slice_mask.len() - 1] = bit_info.ending_bit_mask;
        
        //thne from here do some bit shifting
        //then do an AND MASK
        let mut initial_value = u32::from_slice(slice).unwrap();
        initial_value = initial_value & u32::from_slice(&slice_mask).unwrap();
        if bit_info.starting_bit_mask.trailing_zeros() > 0
        {
            initial_value = initial_value >> bit_info.starting_bit_mask.trailing_zeros();
        }
        initial_value
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
        bit_pack.set_new("Hello", 4, 7);
        
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
        bit_pack.set_new("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("World", 4, 6);

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
        bit_pack.set_new("Hello", 4, 7);
        
        //0b0000_1111

        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

            
        bit_pack.set_new("World", 5, 30);
        
        //0b0000_1111
        //0b0000_0000|0b0000_1111

        //0b0000_0000|0b0000_1111
        //0b0000_0001|0b1111_1111

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b0000_0001,
            starting_array_index_inc: 0,
            ending_array_index_exc: 2
        });
    }

    #[test]
    fn insert_data_three_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set_new("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b1111_0000,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("Zack", 4, 3);

        assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 1,
            ending_array_index_exc: 2
        });
    }

    #[test]
    fn insert_data_four_datas() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set_new("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("World", 4, 6);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b1111_0000,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("Zack", 4, 3);

        assert_eq!(bit_pack.get_bit_info(2), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 1,
            ending_array_index_exc: 2
        });

        bit_pack.set_new("World", 5, 30);

        assert_eq!(bit_pack.get_bit_info(3), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b0000_0001,
            starting_array_index_inc: 1,
            ending_array_index_exc: 3
        });
    }

    #[test]
    fn insert_data_get_one_data() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set_new("Hello", 4, 7);

        assert_eq!(bit_pack.get_u8("Hello"), 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });
    }

    #[test]
    fn insert_two_data_with_overflow_get_second_data() {
        let mut bit_pack : BitPack = BitPack::new();
        bit_pack.set_new("Hello", 4, 7);
        
        assert_eq!(bit_pack.get_bit_info(0), &BitInfo {
            starting_bit_mask : 0b0000_1111,
            ending_bit_mask : 0b0000_1111,
            starting_array_index_inc: 0,
            ending_array_index_exc: 1
        });

        bit_pack.set_new("World", 5, 30);

        assert_eq!(bit_pack.get_bit_info(1), &BitInfo {
            starting_bit_mask : 0b1111_0000,
            ending_bit_mask : 0b0000_0001,
            starting_array_index_inc: 0,
            ending_array_index_exc: 2
        });

        assert_eq!(bit_pack.get_u32("World"), 30);

        // because 30 is represented as 11110 in binary
        // but becuase of overflow element 0 is represented as 1110_0111
        // masking out first 4 and shifting left we get 1110 == 14
        assert_eq!(bit_pack.get_u8("World"), 14);
    }
}