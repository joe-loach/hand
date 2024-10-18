#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Variable {
    /// 0
    Zero,
    /// 1
    One,
    /// label
    Label,
    /// Rn
    Rn,
    /// Rm
    Rm,
    /// Rt
    Rt,
    /// Rd
    Rd,
    /// registers
    RegisterList,
    /// S
    Signed,
    /// cond
    Condition,
    /// stype
    Stype,
    /// imm5
    Imm5,
    /// imm12
    Imm12,
    /// imm24
    Imm24,
    /// Index
    P,
    /// Unsigned / Positive
    U,
    /// Write Back
    W,
}
