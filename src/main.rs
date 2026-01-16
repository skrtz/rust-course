#![allow(unused_variables)]

fn main() {
    /*
    Integer Types
    ---------------------------------------
    Signed Integers: i8, i16, i32, i64, i128, isize
    Unsigned Integers: u8, u16, u32, u64, u128, usize
    ---------------------------------------
    */
    // 8 bits = 1 byte of memory
    let eight_bit_signed: i8 = -128;
    let eight_bit_unsigned: u8 = 255;

    // 16 bits = 2 bytes of memory
    let sixteen_bit_signed: i16 = -32768;
    let sixteen_bit_unsigned: u16 = 65535;

    // 32 bits = 4 bytes of memory
    let thirty_two_bit_signed: i32 = -2147483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    // 64 bits = 8 bytes of memory
    let sixty_four_bit_signed: i64 = -9223372036854775808;
    let sixty_four_bit_unsigned: u64 = 18446744073709551615;

    // 128 bits = 16 bytes of memory
    let one_twenty_eight_bit_signed: i128 = -170141183460469231731687303715884105728;
    let one_twenty_eight_bit_unsigned: u128 = 340282366920938463463374607431768211455;
    
    // Pointer-sized integers
    let pointer_sized_signed: isize = -9223372036854775808;
    let pointer_sized_unsigned: usize = 18446744073709551615;

    /* 
    Memory Units
    ---------------------------------------
    1 Byte (B)      = 8 bits
    1 Kilobyte (KB) = 1,024 Bytes   
    1 Megabyte (MB) = 1,024 Kilobytes
    1 Gigabyte (GB) = 1,024 Megabytes
    ---------------------------------------
    Number Literals    
    ---------------------------------------
    Decimal:        98_222
    Hexadecimal:    0xff_ee
    Octal:         0o77_777
    Binary:        0b1111_0000_1010_1010
    Byte (u8 only): b'A'  (ASCII value for 'A' is 65)
    ---------------------------------------
    */
    let number_literal = 98_222; // Using underscores for readability

}