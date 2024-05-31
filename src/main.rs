use rs_custom_bit_width_packer::{BitPack, BitType, Construct};

fn main () {
    let mut bit_pack = BitPack::new();
    bit_pack.set("Hello", 4, 7);
    bit_pack.set("World", 4, 6);
    
    
    //0b[0110][0111]
    
    //0b0000_0111
    //OR  0b0000_0110
//-----------------
    //0b0000_0111
// 0000_0000_0110
    bit_pack.set("Zack", 4, 6);

    //0b[0000][0111]_0b[0110][0111]
    
    //0b0000_0111
    //OR  0b0000_0110
//-----------------
    //0b0000_0111
// 0000_0000_0110

}