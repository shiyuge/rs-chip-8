// the specification is from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#00E0

mod opcode;
mod sprites;

struct VM {
    /* The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095).
    The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by programs.
    Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536).
    Programs beginning at 0x600 are intended for the ETI 660 computer.
    Memory Map:
    +---------------+= 0xFFF (4095) End of Chip-8 RAM
    |               |
    |               |
    |               |
    |               |
    |               |
    | 0x200 to 0xFFF|
    |     Chip-8    |
    | Program / Data|
    |     Space     |
    |               |
    |               |
    |               |
    +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
    |               |
    |               |
    |               |
    +---------------+= 0x200 (512) Start of most Chip-8 programs
    | 0x000 to 0x1FF|
    | Reserved for  |
    |  interpreter  |
    +---------------+= 0x000 (0) Start of Chip-8 RAM
    */
    memory: [u8; 4096],

    /* Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    There is also a 16-bit register called I.
    This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.

    The VF register should not be used by any program, as it is used as a flag by some instructions.
    */
    registers: [u8; 16],
    i: u16,

    /* Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers.
    When these registers are non-zero, they are automatically decremented at a rate of 60Hz.
    */
    dt: u8, // display timer
    st: u8, // sound timer

    /* There are also some "pseudo-registers" which are not accessable from Chip-8 programs.
    The program counter (PC) should be 16-bit, and is used to store the currently executing address.
    The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    */
    pc: u16,
    sp: u8,

    /* The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to
    when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
    */
    stack: [u16; 16],
}
