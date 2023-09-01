use anyhow::Ok;
use random::Source;

use super::opcode::{Addr, Byte, OpCode, V};

const MEMORY_BYTES: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_LENGTH: usize = 16;

#[derive(Clone)]
pub struct VM {
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
    memory: [u8; MEMORY_BYTES],

    /* Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    There is also a 16-bit register called I.
    This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.

    The VF register should not be used by any program, as it is used as a flag by some instructions.
    */
    registers: [u8; REGISTER_COUNT],
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
    stack: [u16; STACK_LENGTH],

    // screen, random device and so on
    pheriphal: Pheriphal,
}

#[derive(Clone)]
struct Pheriphal {
    random_device: Box<random::Xorshift128Plus>,
}

impl VM {
    pub fn new() -> VM {
        let device = random::default(42);
        let p = Pheriphal {
            random_device: Box::new(device),
        };

        VM {
            memory: [0; MEMORY_BYTES],
            registers: [0; REGISTER_COUNT],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_LENGTH],
            pheriphal: p,
        }
    }

    pub fn execute(&mut self, op: OpCode) -> anyhow::Result<()> {
        match op {
            OpCode::System(nnn) => self.system(nnn),
            OpCode::ClearScreen => self.clearscreen(),
            OpCode::Return => self.execute_return(),
            OpCode::Jump(nnn) => self.jump(nnn),
            OpCode::Call(nnn) => self.execute_call(nnn),
            OpCode::SkipEqual(x, kk) => self.skip_equal(x, kk),
            OpCode::SkipNotEqual(x, kk) => self.skip_not_equal(x, kk),
            OpCode::SkipEqualRegister(x, y) => self.skip_equal_register(x, y),
            OpCode::Load(x, kk) => self.load(x, kk),
            OpCode::Add(x, kk) => self.add(x, kk),
            OpCode::LoadRegister(x, y) => self.load_register(x, y),
            OpCode::OrRegister(x, y) => self.or_register(x, y),
            OpCode::AndRegister(x, y) => self.and_register(x, y),
            OpCode::XorRegister(x, y) => self.xor_register(x, y),
            OpCode::AddRegister(x, y) => self.add_register(x, y),
            OpCode::SubRegister(x, y) => self.sub_register(x, y),
            OpCode::ShrRegister(x, y) => self.shr_register(x, y),
            OpCode::SubNotBorrowRegister(x, y) => self.sub_not_borrow_register(x, y),
            OpCode::ShlRegister(x, y) => self.shl_register(x, y),
            OpCode::SkipNotEqualRegister(x, y) => self.skip_not_equal_register(x, y),
            OpCode::Set(nnn) => self.set(nnn),
            OpCode::JumpV0(nnn) => self.jump_v0(nnn),
            OpCode::Random(x, kk) => self.execute_random(x, kk),
            OpCode::Draw(x, y, nibble) => self.draw(x, y, nibble),
            OpCode::SkipKey(x) => self.key(x),
            OpCode::SkipNotKey(x) => self.skip_not_key(x),
            OpCode::LoadDelayTimer(x) => self.load_dt(x),
            OpCode::LoadKey(x) => self.load_key(x),
            OpCode::SetDelayTimer(x) => self.set_dt(x),
            OpCode::SetSoundTimer(x) => self.set_st(x),
            OpCode::AddI(x) => self.add_i(x),
            OpCode::LoadSprite(x) => self.load_sprite(x),
            OpCode::LoadBCD(x) => self.load_bcd(x),
            OpCode::SaveRegisters(x) => self.save_registers(x),
            OpCode::LoadRegisters(x) => self.load_registers(x),
        }
    }
}

// implementation for opcodes
impl VM {
    fn system(&mut self, nnn: Addr) -> anyhow::Result<()> {
        /* 0nnn - SYS addr
        Jump to a machine code routine at nnn.
        This instruction is only used on the old computers on which Chip-8 was originally implemented.
        It is ignored by modern interpreters.
        */
        Ok(())
    }

    fn clearscreen(&mut self) -> anyhow::Result<()> {
        /* 00E0 - CLS
        Clear the display.
        */
        todo!()
    }

    fn execute_return(&mut self) -> anyhow::Result<()> {
        /* 00EE - RET
        Return from a subroutine.
        The interpreter sets the program counter to the address at the top of the stack,
        then subtracts 1 from the stack pointer.
        */
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
        Ok(())
    }

    fn jump(&mut self, nnn: Addr) -> anyhow::Result<()> {
        /* 1nnn - JP addr
        Jump to location nnn.
        The interpreter sets the program counter to nnn.
        */
        self.pc = nnn.0;
        Ok(())
    }

    fn execute_call(&mut self, nnn: Addr) -> anyhow::Result<()> {
        /* 2nnn - CALL addr
        Call subroutine at nnn.
        The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
        The PC is then set to nnn.
        */
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn.0;
        Ok(())
    }

    fn skip_equal(&mut self, x: V, kk: Byte) -> anyhow::Result<()> {
        /* 3xkk - SE Vx, byte
        Skip next instruction if Vx = kk.
        The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
        */
        if self.registers[x.0 as usize] == kk.0 {
            self.pc += 2;
        }
        Ok(())
    }

    fn skip_not_equal(&mut self, x: V, kk: Byte) -> anyhow::Result<()> {
        /* 4xkk - SNE Vx, byte
        Skip next instruction if Vx != kk.
        The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
        */
        if self.registers[x.0 as usize] != kk.0 {
            self.pc += 2;
        }
        Ok(())
    }

    fn skip_equal_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 5xy0 - SE Vx, Vy
        Skip next instruction if Vx = Vy.
        The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
        */
        if self.registers[x.0 as usize] == self.registers[y.0 as usize] {
            self.pc += 2;
        }
        Ok(())
    }

    // todo 似乎不叫 load，应该叫 set_literal?
    fn load(&mut self, x: V, kk: Byte) -> anyhow::Result<()> {
        /* 6xkk - LD Vx, byte
        Set Vx = kk.
        The interpreter puts the value kk into register Vx.
        */
        self.registers[x.0 as usize] = kk.0;
        Ok(())
    }

    fn add(&mut self, x: V, kk: Byte) -> anyhow::Result<()> {
        /* 7xkk - ADD Vx, byte
        Set Vx = Vx + kk.
        Adds the value kk to the value of register Vx, then stores the result in Vx.
        */
        self.registers[x.0 as usize] += kk.0;
        Ok(())
    }

    // todo 似乎也应该叫 set，copy 之类的？
    fn load_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy0 - LD Vx, Vy
        Set Vx = Vy.
        Stores the value of register Vy in register Vx.
        */
        self.registers[x.0 as usize] = self.registers[y.0 as usize];
        Ok(())
    }

    fn or_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy1 - OR Vx, Vy
        Set Vx = Vx OR Vy.
        Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
        A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
        then the same bit in the result is also 1. Otherwise, it is 0.
        */

        self.registers[x.0 as usize] = self.registers[x.0 as usize] | self.registers[y.0 as usize];
        Ok(())
    }

    fn and_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy2 - AND Vx, Vy
        Set Vx = Vx AND Vy.
        Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
        A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
         then the same bit in the result is also 1. Otherwise, it is 0.
        */
        self.registers[x.0 as usize] = self.registers[x.0 as usize] & self.registers[y.0 as usize];
        Ok(())
    }

    fn xor_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy3 - XOR Vx, Vy
        Set Vx = Vx XOR Vy.
        Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
        */
        self.registers[x.0 as usize] = self.registers[x.0 as usize] ^ self.registers[y.0 as usize];
        Ok(())
    }

    fn add_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy4 - ADD Vx, Vy
        Set Vx = Vx + Vy, set VF = carry.
        The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
        */
        let r = self.registers[x.0 as usize] as u16 + self.registers[y.0 as usize] as u16;
        if r > u8::MAX as u16 {
            self.registers[0x0f] = 1;
        } else {
            self.registers[0x0f] = 0;
        }
        self.registers[x.0 as usize] = r as u8; // todo verify
        Ok(())
    }

    fn sub_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy5 - SUB Vx, Vy
        Set Vx = Vx - Vy, set VF = NOT borrow.
        If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
        */
        if self.registers[x.0 as usize] > self.registers[y.0 as usize] {
            self.registers[0x0f] = 1;
        } else {
            self.registers[0x0f] = 0;
        }
        self.registers[x.0 as usize] = self.registers[x.0 as usize] - self.registers[y.0 as usize];
        Ok(())
    }

    fn shr_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy6 - SHR Vx {, Vy}
        Set Vx = Vx SHR 1.
        If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
        */
        if self.registers[x.0 as usize] & 0b0000_0001 == 1 {
            self.registers[0x0f] = 1;
        } else {
            self.registers[0x0f] = 0;
        }
        self.registers[x.0 as usize] = self.registers[x.0 as usize] >> 1;
        Ok(())
    }

    fn sub_not_borrow_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xy7 - SUBN Vx, Vy
        Set Vx = Vy - Vx, set VF = NOT borrow.
        If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
        */
        if self.registers[y.0 as usize] > self.registers[x.0 as usize] {
            self.registers[0x0f] = 1;
        } else {
            self.registers[0x0f] = 0;
        }
        self.registers[x.0 as usize] = self.registers[y.0 as usize] - self.registers[x.0 as usize];
        Ok(())
    }

    fn shl_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 8xyE - SHL Vx {, Vy}
        Set Vx = Vx SHL 1.
        If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
        */
        if self.registers[x.0 as usize] & 0b1000_0000 == 1 {
            self.registers[0x0f] = 1;
        } else {
            self.registers[0x0f] = 0;
        }
        self.registers[x.0 as usize] = self.registers[x.0 as usize] << 1;
        Ok(())
    }

    fn skip_not_equal_register(&mut self, x: V, y: V) -> anyhow::Result<()> {
        /* 9xy0 - SNE Vx, Vy
        Skip next instruction if Vx != Vy.
        The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
        */
        if self.registers[x.0 as usize] != self.registers[y.0 as usize] {
            self.pc += 2;
        }
        Ok(())
    }

    fn set(&mut self, nnn: Addr) -> anyhow::Result<()> {
        /* Annn - LD I, addr
        Set I = nnn.
        The value of register I is set to nnn.
        */
        self.i = nnn.0;
        Ok(())
    }

    fn jump_v0(&mut self, nnn: Addr) -> anyhow::Result<()> {
        /* Bnnn - JP V0, addr
        Jump to location nnn + V0.
        The program counter is set to nnn plus the value of V0.
        */
        self.pc = self.registers[0] as u16 + nnn.0;
        Ok(())
    }

    fn execute_random(&mut self, x: V, kk: Byte) -> anyhow::Result<()> {
        /* Cxkk - RND Vx, byte
        Set Vx = random byte AND kk.
        The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
        */
        let r = self.pheriphal.random_device.read::<u8>();
        self.registers[x.0 as usize] = r & kk.0;
        Ok(())
    }

    fn draw(&mut self, x: V, y: V, nibble: u8) -> anyhow::Result<()> {
        /* Dxyn - DRW Vx, Vy, nibble
        Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
        */
        todo!()
    }

    fn key(&mut self, x: V) -> anyhow::Result<()> {
        /* Ex9E - SKP Vx
        Skip next instruction if key with the value of Vx is pressed.
        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
        */
        todo!()
    }

    fn skip_not_key(&mut self, x: V) -> anyhow::Result<()> {
        /* ExA1 - SKNP Vx
        Skip next instruction if key with the value of Vx is not pressed.
        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
        */
        todo!()
    }

    fn load_dt(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx07 - LD Vx, DT
        Set Vx = delay timer value.
        The value of DT is placed into Vx.
        */
        self.registers[x.0 as usize] = self.dt;
        Ok(())
    }

    fn load_key(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx0A - LD Vx, K
        Wait for a key press, store the value of the key in Vx.
        All execution stops until a key is pressed, then the value of that key is stored in Vx.
        */
        todo!()
    }

    fn set_dt(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx15 - LD DT, Vx
        Set delay timer = Vx.
        DT is set equal to the value of Vx.
        */
        self.dt = self.registers[x.0 as usize];
        Ok(())
    }

    fn set_st(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx18 - LD ST, Vx
        Set sound timer = Vx.
        ST is set equal to the value of Vx.
        */
        self.st = self.registers[x.0 as usize];
        Ok(())
    }

    fn add_i(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx1E - ADD I, Vx
        Set I = I + Vx.
        The values of I and Vx are added, and the results are stored in I.
        */
        self.i = self.i + self.registers[x.0 as usize] as u16;
        Ok(())
    }

    fn load_sprite(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx29 - LD F, Vx
        Set I = location of sprite for digit Vx.
        The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
        See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
        */

        todo!()
    }

    fn load_bcd(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx33 - LD B, Vx
        Store BCD representation of Vx in memory locations I, I+1, and I+2.
        The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
        */
        todo!()
    }

    fn save_registers(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx55 - LD [I], Vx
        Store registers V0 through Vx in memory starting at location I.
        The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
        */
        for (offset, index) in (0..=x.0).enumerate() {
            self.memory[self.i as usize + offset] = self.registers[index as usize];
        }
        Ok(())
    }

    fn load_registers(&mut self, x: V) -> anyhow::Result<()> {
        /* Fx65 - LD Vx, [I]
        Read registers V0 through Vx from memory starting at location I.
        The interpreter reads values from memory starting at location I into registers V0 through Vx.
        */
        for (offset, index) in (0..=x.0).enumerate() {
            self.registers[index as usize] = self.memory[self.i as usize + offset];
        }
        Ok(())
    }
}
