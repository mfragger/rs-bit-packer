use rs_custom_bit_width_packer::{BitPack, BitType, Construct};

fn main () {
    let mut bit_pack = BitPack::new();
    bit_pack.set("Hello", 4, 7);
    bit_pack.set("World", 4, 6);
    
    
    //0b[0110][0111]      
    //[0] 103

    //starting bit = 0
    //ending bit = 5
    //starting arr = 0
    //ending arr = 1

    //starting bit = 5
    //ending bit = 0
    //starring arr =0
    //ending arr = 1

    bit_pack.set("Zack", 4, 3);
    // 0b0000_[0011]_0b[0110][0111]
    //[0] 103
    //[1] 3

    //starting bit = 0
    //ending bit = 5
    //starting arr = 1
    //ending arr = 2

    bit_pack.set("Markd", 5, 30);

    println!("Done!");
    // 0b0000_000[1_0b1110]_[0011]_0b[0110][0111]
    // [0] 103
    // [1] 227
    // 0b0000_0011_0b01100111
    // 0b
    // [2] 1

    //starting bit = 5
    //ending bit = 1
    //starting arr = 1
    //ending arr = 3
}