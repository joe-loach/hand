use crate::variable::{Variable, VariableDef};

/// Describes what variables are needed and which bits they fill.
pub struct Schema {
    pub(crate) base: u32,
    // slots for 8 variables
    pub(crate) variables: [Option<VariableDef>; 8],
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schema(")?;
        write!(f, "{:032b}", self.base)?;
        if self.variables.first().is_some_and(|it| it.is_some()) {
            write!(f, ", {{ ")?;
            for def in self.variables.iter().filter(|it| it.is_some()).rev() {
                let Some(VariableDef { name, high, low }) = def else {
                    unreachable!()
                };
                match name {
                    Variable::Rn => write!(f, "Rn")?,
                    Variable::Rm => write!(f, "Rm")?,
                    Variable::Rt => write!(f, "Rt")?,
                    Variable::Rd => write!(f, "Rd")?,
                    Variable::RegisterList => write!(f, "registers")?,
                    Variable::Signed => write!(f, "S")?,
                    Variable::Condition => write!(f, "cond")?,
                    Variable::Stype => write!(f, "stype")?,
                    Variable::Imm5 => write!(f, "imm5")?,
                    Variable::Imm12 => write!(f, "imm12")?,
                    Variable::Imm24 => write!(f, "imm24")?,
                }
                write!(f, "({},{})", high, low)?;
                write!(f, " ")?;
            }
            write!(f, "}}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

pub const fn arg(name: u32, position: u32) -> u64 {
    (name as u64) << 32 | position as u64
}

pub const fn schema<const LN: usize>(layout: [u32; LN]) -> Schema {
    assert!(LN <= 32);

    let mut variables: [Option<VariableDef>; 8] = [const { None }; 8];

    let mut base = 0x0;

    let mut var_idx = 0;
    let mut bit = 0;

    let mut i = LN;
    while i != 0 {
        i = i.saturating_sub(1);
        let curr = layout[i];

        let (bit_len, set_var) = match curr {
            0 => (1, None),
            1 => {
                base |= 1 << bit;
                (1, None)
            }
            COND => (4, Some(Variable::Condition)),
            S => (1, Some(Variable::Signed)),
            IMM12 => (12, Some(Variable::Imm12)),
            x if x == R('n') => (4, Some(Variable::Rn)),
            x if x == R('m') => (4, Some(Variable::Rm)),
            x if x == R('d') => (4, Some(Variable::Rd)),
            x if x == R('t') => (4, Some(Variable::Rt)),
            _ => panic!("Unknown bit pattern in Schema"),
        };

        if let Some(name) = set_var {
            variables[var_idx] = Some(VariableDef {
                name,
                high: bit + bit_len,
                low: bit,
            });
            var_idx += 1;
        }

        bit += bit_len;
    }

    assert!(bit == 32, "Incorrect number of bits to build a Schema");

    Schema { base, variables }
}

pub const COND: u32 = 1_u32 << 31;

pub const S: u32 = 1_u32 << 30;

pub const IMM12: u32 = 1_u32 << 29;

#[allow(non_snake_case)]
pub const fn R(x: char) -> u32 {
    assert!(x.is_ascii());
    let x = (x as u8) as u32;
    (1_u32 << 28) + x
}
