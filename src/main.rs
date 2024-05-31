use rs_custom_bit_width_packer::{BitPack, BitType, Construct};

fn main () {
    // const LOW_3_BITS_MASK: u8 = 0b0000_0000;
    // let num = 255u8;
    // let high = num >> 4; 


    // let sam1 = 15u8;
    // let sam2 = 15u128;

    // // println!("{}", u8::from(high & LOW_3_BITS_MASK));
    // println!("How many in u8: {}, How many in u128 {}", sam1.leading_zeros(), sam2.leading_zeros());

    // let initial_value: u128 = 0b0000_0011;
    // // 0b0000_0[011];
    // // 0b0000_0[011]  0b1111_1111
    // // 0b0000_0[011]
    // // 0b0000_0000;

    // let mask_type: BitType = match 128 - initial_value.leading_zeros() {
    //     1..=8 => BitType::U8(0),
    //     9..=16 => BitType::U16(0),
    //     17..=32 => BitType::U32(0),
    //     33..=64 => BitType::U16(0),
    //     65..=128 => BitType::U128(0),
    //     _ => panic!("Value out of range"),
    // };
    // println!("{},{:?}", 128-initial_value.leading_zeros(), mask_type);

    let bit_pack = BitPack::new();
    bit_pack.set("Hello", 4, 15);


}
//Declare a size ==== 3
    //store mask = 0b0000_0111
    // 0b0000_0333
    //Set a pointer = 0
    //set NEXT POINTER = 4
//Set a num with size 4
    //Store mask =1 0b000_111
    //0b0111_1333
    //bit shift to === 3
    //set a pointer = 4
    //set NEXT pointer = 7
//Declare a size === 3
    //0b000_0022_0b2111_1333
    //set pointer = 8
    //store mask = 0b000_0111
    //set next pointer = 3