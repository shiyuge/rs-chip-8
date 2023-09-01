pub struct Byte(pub u8); // kk

pub struct Addr(pub u16); // nnn

pub struct V(pub u8); // 0x00 - 0x0f

pub enum OpCode {
    /* 0nnn - SYS addr
    Jump to a machine code routine at nnn.
    This instruction is only used on the old computers on which Chip-8 was originally implemented.
    It is ignored by modern interpreters.
    */
    System(Addr),

    /* 00E0 - CLS
    Clear the display.
    */
    ClearScreen,

    /* 00EE - RET
    Return from a subroutine.
    The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    */
    Return,

    /* 1nnn - JP addr
    Jump to location nnn.
    The interpreter sets the program counter to nnn.
    */
    Jump(Addr),

    /* 2nnn - CALL addr
    Call subroutine at nnn.
    The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    */
    Call(Addr),

    /* 3xkk - SE Vx, byte
    Skip next instruction if Vx = kk.
    The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    */
    SkipEqual(V, Byte),

    /* 4xkk - SNE Vx, byte
    Skip next instruction if Vx != kk.
    The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    */
    SkipNotEqual(V, Byte),

    /* 5xy0 - SE Vx, Vy
    Skip next instruction if Vx = Vy.
    The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    */
    SkipEqualRegister(V, V),

    /* 6xkk - LD Vx, byte
    Set Vx = kk.
    The interpreter puts the value kk into register Vx.
    */
    Load(V, Byte),

    /* 7xkk - ADD Vx, byte
    Set Vx = Vx + kk.
    Adds the value kk to the value of register Vx, then stores the result in Vx.
    */
    Add(V, Byte),

    /* 8xy0 - LD Vx, Vy
    Set Vx = Vy.
    Stores the value of register Vy in register Vx.
    */
    LoadRegister(V, V),

    /* 8xy1 - OR Vx, Vy
    Set Vx = Vx OR Vy.
    Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    */
    OrRegister(V, V),

    /* 8xy2 - AND Vx, Vy
    Set Vx = Vx AND Vy.
    Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    */
    AndRegister(V, V),

    /* 8xy3 - XOR Vx, Vy
    Set Vx = Vx XOR Vy.
    Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    */
    XorRegister(V, V),

    /* 8xy4 - ADD Vx, Vy
    Set Vx = Vx + Vy, set VF = carry.
    The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    */
    AddRegister(V, V),

    /* 8xy5 - SUB Vx, Vy
    Set Vx = Vx - Vy, set VF = NOT borrow.
    If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    */
    SubRegister(V, V),

    /* 8xy6 - SHR Vx {, Vy}
    Set Vx = Vx SHR 1.
    If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    */
    ShrRegister(V, V),

    /* 8xy7 - SUBN Vx, Vy
    Set Vx = Vy - Vx, set VF = NOT borrow.
    If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    */
    SubNotBorrowRegister(V, V),

    /* 8xyE - SHL Vx {, Vy}
    Set Vx = Vx SHL 1.
    If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    */
    ShlRegister(V, V),

    /* 9xy0 - SNE Vx, Vy
    Skip next instruction if Vx != Vy.
    The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    */
    SkipNotEqualRegister(V, V),

    /* Annn - LD I, addr
    Set I = nnn.
    The value of register I is set to nnn.
    */
    Set(Addr),

    /* Bnnn - JP V0, addr
    Jump to location nnn + V0.
    The program counter is set to nnn plus the value of V0.
    */
    JumpV0(Addr),

    /* Cxkk - RND Vx, byte
    Set Vx = random byte AND kk.
    The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    */
    Random(V, Byte),

    /* Dxyn - DRW Vx, Vy, nibble
    Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    */
    Draw(V, V, u8),

    /* Ex9E - SKP Vx
    Skip next instruction if key with the value of Vx is pressed.
    Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    */
    SkipKey(V),

    /* ExA1 - SKNP Vx
    Skip next instruction if key with the value of Vx is not pressed.
    Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    */
    SkipNotKey(V),

    /* Fx07 - LD Vx, DT
    Set Vx = delay timer value.
    The value of DT is placed into Vx.
    */
    LoadDelayTimer(V),

    /* Fx0A - LD Vx, K
    Wait for a key press, store the value of the key in Vx.
    All execution stops until a key is pressed, then the value of that key is stored in Vx.
    */
    LoadKey(V),

    /* Fx15 - LD DT, Vx
    Set delay timer = Vx.
    DT is set equal to the value of Vx.
    */
    SetDelayTimer(V),

    /* Fx18 - LD ST, Vx
    Set sound timer = Vx.
    ST is set equal to the value of Vx.
    */
    SetSoundTimer(V),

    /* Fx1E - ADD I, Vx
    Set I = I + Vx.
    The values of I and Vx are added, and the results are stored in I.
    */
    AddI(V),

    /* Fx29 - LD F, Vx
    Set I = location of sprite for digit Vx.
    The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    */
    LoadSprite(V),

    /* Fx33 - LD B, Vx
    Store BCD representation of Vx in memory locations I, I+1, and I+2.
    The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    */
    LoadBCD(V),

    /* Fx55 - LD [I], Vx
    Store registers V0 through Vx in memory starting at location I.
    The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    */
    SaveRegisters(V),

    /* Fx65 - LD Vx, [I]
    Read registers V0 through Vx from memory starting at location I.
    The interpreter reads values from memory starting at location I into registers V0 through Vx.
    */
    LoadRegisters(V),
}

// TODO as cast could fail, and second cannot be always confined to u4

impl TryFrom<u16> for OpCode {
    type Error = String; // todo std::err

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let first = (value >> (3 * 4)) as u8;
        let second = ((value & 0x0100) >> (2 * 4)) as u8;
        let thrid = ((value & 0x0010) >> 4) as u8;
        let fourth: u16 = value & 0x001;
        let fourth = fourth as u8;

        let nnn = value & 0x0111;
        let kk = (value & 0x0011) as u8;

        match first {
            0x00 => {
                if value == 0x00E0 {
                    return Ok(OpCode::ClearScreen);
                } else if value == 0x00EE {
                    return Ok(OpCode::Return);
                }
                Ok(OpCode::System(Addr(nnn)))
            }
            0x01 => Ok(OpCode::Jump(Addr(nnn))),
            0x02 => Ok(OpCode::Call(Addr(nnn))),
            0x03 => Ok(OpCode::SkipEqual(V(second), Byte(kk))),
            0x04 => Ok(OpCode::SkipNotEqual(V(second), Byte(kk))),
            0x05 => {
                if fourth != 0 {
                    return Err("fourth not zero".to_owned());
                }
                Ok(OpCode::SkipEqualRegister(V(second), V(thrid)))
            }
            0x06 => Ok(OpCode::Load(V(second), Byte(kk))),
            0x07 => Ok(OpCode::Add(V(second), Byte(kk))),
            0x08 => match fourth {
                0x00 => Ok(OpCode::LoadRegister(V(second), V(thrid))),
                0x01 => Ok(OpCode::OrRegister(V(second), V(thrid))),
                0x02 => Ok(OpCode::AndRegister(V(second), V(thrid))),
                0x03 => Ok(OpCode::XorRegister(V(second), V(thrid))),
                0x04 => Ok(OpCode::AddRegister(V(second), V(thrid))),
                0x05 => Ok(OpCode::SubRegister(V(second), V(thrid))),
                0x06 => Ok(OpCode::ShrRegister(V(second), V(thrid))),
                0x07 => Ok(OpCode::SubNotBorrowRegister(V(second), V(thrid))),
                0x0E => Ok(OpCode::ShlRegister(V(second), V(thrid))),
                _ => Err("0x8xx fail".to_owned()),
            },
            0x09 => {
                if fourth != 0 {
                    return Err("fourth is not zero".to_owned());
                }
                Ok(OpCode::SkipNotEqualRegister(V(second), V(thrid)))
            }
            0x0a => Ok(OpCode::Set(Addr(nnn))),
            0x0b => Ok(OpCode::JumpV0(Addr(nnn))),
            0x0c => Ok(OpCode::Random(V(second), Byte(kk))),
            0x0d => Ok(OpCode::Draw(V(second), V(thrid), fourth)),
            0x0e => match kk {
                0x9e => Ok(OpCode::SkipKey(V(second))),
                0xa1 => Ok(OpCode::SkipNotKey(V(second))),
                _ => Err("neither Ex9E nor ExA1".to_owned()),
            },
            0x0f => match kk {
                0x07 => Ok(OpCode::LoadDelayTimer(V(second))),
                0x0a => Ok(OpCode::LoadKey(V(second))),
                0x15 => Ok(OpCode::SetDelayTimer(V(second))),
                0x18 => Ok(OpCode::SetSoundTimer(V(second))),
                0x1e => Ok(OpCode::AddI(V(second))),
                0x29 => Ok(OpCode::LoadSprite(V(second))),
                0x33 => Ok(OpCode::LoadBCD(V(second))),
                0x55 => Ok(OpCode::SaveRegisters(V(second))),
                0x65 => Ok(OpCode::LoadRegisters(V(second))),
                _ => Err("0xfxx fail".to_owned()),
            },
            _ => Err("first value fail".to_owned()),
        }
    }
}

impl Into<String> for OpCode {
    fn into(self) -> String {
        match self {
            OpCode::System(nnn) => format!("SYS {}", nnn.0),
            OpCode::ClearScreen => "CLS".to_owned(),
            OpCode::Return => "RET".to_owned(),
            OpCode::Jump(nnn) => format!("JP {}", nnn.0),
            OpCode::Call(nnn) => format!("CALL {}", nnn.0),
            OpCode::SkipEqual(x, kk) => format!("SE V{} {}", x.0, kk.0),
            OpCode::SkipNotEqual(x, kk) => format!("SNE V{} {}", x.0, kk.0),
            OpCode::SkipEqualRegister(x, y) => format!("SN V{} V{}", x.0, y.0),
            OpCode::Load(x, kk) => format!("LD V{} {}", x.0, kk.0),
            OpCode::Add(x, kk) => format!("ADD V{} {}", x.0, kk.0),
            OpCode::LoadRegister(x, y) => format!("LD V{} V{}", x.0, y.0),
            OpCode::OrRegister(x, y) => format!("OR V{} V{}", x.0, y.0),
            OpCode::AndRegister(x, y) => format!("AND V{} V{}", x.0, y.0),
            OpCode::XorRegister(x, y) => format!("XOR V{} V{}", x.0, y.0),
            OpCode::AddRegister(x, y) => format!("ADD V{} V{}", x.0, y.0),
            OpCode::SubRegister(x, y) => format!("SUB V{} V{}", x.0, y.0),
            OpCode::ShrRegister(x, y) => format!("SHR V{} V{}", x.0, y.0),
            OpCode::SubNotBorrowRegister(x, y) => format!("SUBN V{} V{}", x.0, y.0),
            OpCode::ShlRegister(x, y) => format!("SHL V{} {{, V{}}}", x.0, y.0),
            OpCode::SkipNotEqualRegister(x, y) => format!("SNE V{}, V{}", x.0, y.0),
            OpCode::Set(nnn) => format!("LD I, {}", nnn.0),
            OpCode::JumpV0(nnn) => format!("JP V0, {}", nnn.0),
            OpCode::Random(x, kk) => format!("RND V{}, {}", x.0, kk.0),
            OpCode::Draw(x, y, nibble) => format!("DRW V{}, V{}, {}", x.0, y.0, nibble),
            OpCode::SkipKey(x) => format!("SKP V{}", x.0),
            OpCode::SkipNotKey(x) => format!("SKNP V{}", x.0),
            OpCode::LoadDelayTimer(x) => format!("LD V{}, DT", x.0),
            OpCode::LoadKey(x) => format!("LD V{}, K", x.0),
            OpCode::SetDelayTimer(x) => format!("LD DT, V{}", x.0),
            OpCode::SetSoundTimer(x) => format!("LD ST, V{}", x.0),
            OpCode::AddI(x) => format!("ADD I, V{}", x.0),
            OpCode::LoadSprite(x) => format!("LD F, V{}", x.0),
            OpCode::LoadBCD(x) => format!("LD B, V{}", x.0),
            OpCode::SaveRegisters(x) => format!("LD [I], V{}", x.0),
            OpCode::LoadRegisters(x) => format!("LD V{}, [I]", x.0),
        }
    }
}

// Do not create these yet

// impl Into<u16> for OpCode {
//     fn into(self) -> u16 {
//         match self {
//             OpCode::System(addr) => todo!(),
//             OpCode::ClearScreen => 0x00E0,
//             OpCode::Return => 0x00EE,
//             OpCode::Jump(addr) => todo!(),
//             OpCode::Call(addr) => todo!(),
//             OpCode::SkipEqual(x, kk) => todo!(),
//             OpCode::SkipNotEqual(x, kk) => todo!(),
//             OpCode::SkipEqualRegister(x, y) => todo!(),
//             OpCode::Load(x, kk) => todo!(),
//             OpCode::Add(x, kk) => todo!(),
//             OpCode::LoadRegister(x, y) => todo!(),
//             OpCode::OrRegister(x, y) => todo!(),
//             OpCode::AndRegister(x, y) => todo!(),
//             OpCode::XorRegister(x, y) => todo!(),
//             OpCode::AddRegister(x, y) => todo!(),
//             OpCode::SubRegister(x, y) => todo!(),
//             OpCode::ShrRegister(x, y) => todo!(),
//             OpCode::SubNotBorrowRegister(x, y) => todo!(),
//             OpCode::ShlRegister(x, y) => todo!(),
//             OpCode::SkipNotEqualRegister(x, y) => todo!(),
//             OpCode::Set(addr) => todo!(),
//             OpCode::JumpV0(addr) => todo!(),
//             OpCode::Random(x, kk) => todo!(),
//             OpCode::Draw(x, y, nibble) => todo!(),
//             OpCode::SkipKey(x) => todo!(),
//             OpCode::SkipNotKey(x) => todo!(),
//             OpCode::LoadDelayTimer(x) => todo!(),
//             OpCode::LoadKey(x) => todo!(),
//             OpCode::SetDelayTimer(x) => todo!(),
//             OpCode::SetSoundTimer(x) => todo!(),
//             OpCode::AddI(x) => todo!(),
//             OpCode::LoadSprite(x) => todo!(),
//             OpCode::LoadBCD(x) => todo!(),
//             OpCode::SaveRegisters(x) => todo!(),
//             OpCode::LoadRegisters(x) => todo!(),
//         }
//     }
// }

// impl TryFrom<&str> for OpCode {
//     type Error = String; // todo std::err

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }
