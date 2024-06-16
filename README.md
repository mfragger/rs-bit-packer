# rs-bit-packer

## Overview
A simple project that stores a number with arbitrary size of bits into an array of u8's.

### Putting it in

Example: Assume that we have the following pseudo code declaration.

```rs
bit_pack.set_new(name: "Hello", to_alloc_size: 4, initial_data: 7)
```

This tells the bit-packer to store the number 7 onto the array of u8s with size 4.

``to_alloc_size`` means how many bits does it need, since we have 7 as an initial value, it can fit with a size of 4 where the maximum value is 15.

In binary it would look like this.

```rs
0000[0111]
```

The brackets shows how many bits we've allocated, in this case 4.

When we declare a new variable again, we simply have store the value onto the remaining un-allocated bits.

like so:

```rs
//from earlier
bit_pack.set_new(name: "Hello", to_alloc_size: 4, initial_data: 7)

//result:
0000[0111]

//set another one
bit_pack.set_new(name: "World", to_alloc_size: 4, initial_data: 6)

//result:
[0110][0111]

```
Mind you, this is still element 0, of the vector of u8's.

Next, what if we declared an allocation that's bigger than the un-allocated bits?

Answer: we simply create a new element, in rust we just ``push(0)`` and put the overflown bits there.

```rs
//from earlier
bit_pack.set_new(name: "Hello", to_alloc_size: 4, initial_data: 7)

//result: 4-bits are remaining to be allocated.
0000[0111]

//set another one
bit_pack.set_new(name: "Word", to_alloc_size: 5, initial_data: 30)

//result: comma denotes a separation of elements.
000000[1,1110][0111]

```

ps: read the bits from right to left.

As you can see from the above, with only 4 bits remaining to be allocated, we simply store what we can onto the remainig bits, create a new element, then store the overflow bits into the new element.

### Taking it out
Now taking it out is rather straightforward but isn't as intuitive as putting it in.

Consider the following pseudo-code:

```rs
bit_pack.get::<u8>(name: "Hello")
```

As you can see, we're declaring the type of the number into the generic function ``get``.

While arbritrary bit sizes are cool and all, we still need to play with rust's known primitive types.

Will this cause some data loss when we don't declare the right type? 

Of course.

Consider the following.

```rs
//from earlier
bit_pack.set_new(name: "Hello", to_alloc_size: 4, initial_data: 7)

//result: 4-bits are remaining to be allocated.
0000[0111]

//set another one
bit_pack.set_new(name: "Word", to_alloc_size: 5, initial_data: 30)

//result: comma denotes a separation of elements.
000000[1,1110][0111]

bit_pack.get::<u16>(name: "Word") //outputs as 30

bit_pack.get::<u8>(name: "Word") //outputs as 30
```

As you can see, with an ``alloc size`` of 5, it can fit into u8 and u16 so it can be casted as u8 or u16.

## Motivation

Because I can.

And also the idea spawned from wanting to create custom bit widths that's found in C but it evolved into some sort of packer.

And also it was fun to do this.