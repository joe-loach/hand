//! Common Immediate Representation

#[cfg(feature = "derive")]
extern crate structure_derive;

pub mod structured;

pub trait Convert {
    fn to_cir(&self) -> Vec<CIR>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CIR {
    Instruction(u32),
    Char(char),
    Register(u32),
    RegisterList(u16),
    Condition(Condition),
    Shift(Shift),
    Number(u32),
    Label(i32),
    OffsetAddress,
    PreIndexAddress,
    PostIndexAddress,
    Bang,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shift {
    /// Logical shift left
    LSL,
    /// Logical shift right
    LSR,
    /// Arithmetic shift right
    ASR,
    /// Rotate right
    ROR,
    /// Special case of ROR when the immediate = 0
    RRX,
}

impl Default for Shift {
    fn default() -> Self {
        Self::LSL
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Condition {
    /// Equal
    EQ = 0b0000,
    /// Not equal
    NE = 0b0001,
    /// Carry set
    CS = 0b0010,
    /// Carry clear
    CC = 0b0011,
    /// Minus, negative
    MI = 0b0100,
    /// Plus, positive or zero
    PL = 0b0101,
    /// Overflow
    VS = 0b0110,
    /// No overflow
    VC = 0b0111,
    /// Unsigned higher
    HI = 0b1000,
    /// Unsigned lower or same
    LS = 0b1001,
    /// Signed greater than or equal
    GE = 0b1010,
    /// Signed less than
    LT = 0b1011,
    /// Signed greater than
    GT = 0b1100,
    /// Signed less than or equal
    LE = 0b1101,
    /// Always (unconditional)
    AL = 0b1110,
}

impl Default for Condition {
    fn default() -> Self {
        Self::AL
    }
}
