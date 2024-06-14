use rs_custom_bit_width_packer::{BitPack, Construct};

fn main () {
    let mut bit_pack = BitPack::new();
    bit_pack.set_new("Hello", 4, 7);
    bit_pack.set_new("World", 4, 6);

    //store both array 0
    //0b0000_0111
    //0b0000_0110
//--------------------
    //0b[0110][0111] <---say I want to get "World" 
//AND 0b[1111][0000] <--count trailing zeroes based on masking info
    //--------------------
    //0b[0110][0000] >> 4 
    //--------------------
//    0b0000[0110]


//How to store a 5 sized bits on a space with 4 bits
//source                0b0001_1110
//target                0b0000_0111
//----------------------------------
//Result    0b0000_0001|0b1110_0111

//Get 4 bits at a time and put into an array
//0b0001_1110 -> 0b0000_[1110] | 0b000[1]_0000 

//put into result:
// take 0th:
//                      0b0000_1110
//Shift 4 to left       0b1110_0000 
//AND MASK with target  0b0000_0111
//partial result        0b1110_0111
//Create mask           0b1111_0000
//set last remaining bit size: 0
//is lrbs == 0?         YES
//do we have more bits? YES
//push      0b0000_0000|0b1110_0111 <-- Notice we don't count for now since we're going to insert in anyway.
//take 1th:
//          0b0001_0000
//Shift 4 right
//          0b0000_0001
//AND MASK  0b0000_0001|0b1110_0111
//



//Another scneario, where the 2nd number I stored has a leading bit to the next array
    //0b000[1_0b1110]0111
//AND 0b000[1_0b1111]0000  <-- 4 trailing, 2 arrays with mask 0b0000_0001 and 0b1111_000
//------------------------
//    0b000[1_0b1110]0000 
//Bring down 0th index:
//            0b1110_0000 >> 4 <-- Should be casted as creating size
//--------------------------------------
//            0b0000_1110
//Create result variable, it should be the same size as the creating size.
//OR          0b0000_0000
//--------------------------------------
//            0b0000_1110
//Bring down 1st index:
//            0b0000_0001 << 4 <-- Should be casted as creating size
//--------------------------------------
//            0b0001_0000
//OR with result
//--------------------------------------
//            0b0001_1110 <-- reconstructed


    //[0] 103

    //starting bit = 0
    //ending bit = 5
    //starting arr = 0
    //ending arr = 1

    //starting bit = 5
    //ending bit = 0
    //starring arr =0
    //ending arr = 1

    // bit_pack.set("Zack", 4, 3);
    // 0b0000_[0011]_0b[0110][0111]
    //[0] 103
    //[1] 3

    //starting bit = 0
    //ending bit = 5
    //starting arr = 1
    //ending arr = 2

    // bit_pack.set("Markd", 5, 30);

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


