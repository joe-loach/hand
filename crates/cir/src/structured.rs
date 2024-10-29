mod buffer;

use std::marker::PhantomData;

pub use buffer::Buffer;

use crate::CIR;

pub fn parse_from_args<T: Parse>(cir: &[CIR]) -> Option<T> {
    let mut buffer = Buffer::new(cir);
    // skip till arguments
    while let Some(CIR::Char(_)) = buffer.peek() {
        buffer.bump();
    }
    let res = T::parse(&mut buffer)?;
    assert!(buffer.is_empty());
    Some(res)
}

macro_rules! match_buffer {
    ($buffer:ident: $pattern:pat => $res:expr) => {{
        match $buffer.peek()? {
            $pattern => {
                $buffer.bump();
                Some($res)
            }
            _ => None,
        }
    }};
}

pub trait Parse {
    fn parse(buffer: &mut Buffer) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Label(pub u32, pub bool);
#[derive(Debug)]
pub struct Condition(pub crate::Condition);
#[derive(Debug)]
pub struct Register<T: RegName>(pub u32, PhantomData<T>);
#[derive(Debug)]
pub struct RegisterList(pub u16);
#[derive(Debug)]
pub struct Number<const BITS: u8>(pub u32);
#[derive(Debug)]
pub struct Shift(pub crate::Shift);

#[derive(Debug)]
pub struct Offset;
#[derive(Debug)]
pub struct PreIndex;
#[derive(Debug)]
pub struct PostIndex;
#[derive(Debug)]
pub struct Address<T>(PhantomData<T>);

#[derive(Debug)]
pub struct Bang;

impl Parse for Label {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Label(lbl) => {
            let signed = lbl.is_negative();
            // value
            if signed {
                Self(lbl.wrapping_neg() as u32, signed)
            } else {
                Self(lbl as u32, signed)
            }
        })
    }
}

impl Parse for Condition {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Condition(cond) => Self(cond))
    }
}

impl Parse for RegisterList {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::RegisterList(mask) => Self(mask))
    }
}

impl<const BITS: u8> Parse for Number<BITS> {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Number(number) => Self(number & ((1 << BITS) - 1)))
    }
}

impl Parse for Shift {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Shift(shift) => Self(shift))
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::D {}
    impl Sealed for super::N {}
    impl Sealed for super::M {}
    impl Sealed for super::R {}
    impl Sealed for super::S {}
    impl Sealed for super::T {}
}

pub trait RegName: private::Sealed {}

#[derive(Debug)]
pub struct D;
#[derive(Debug)]
pub struct N;
#[derive(Debug)]
pub struct M;
#[derive(Debug)]
pub struct R;
#[derive(Debug)]
pub struct S;
#[derive(Debug)]
pub struct T;

impl RegName for D {}
impl RegName for N {}
impl RegName for M {}
impl RegName for R {}
impl RegName for S {}
impl RegName for T {}

impl<T: RegName> Parse for Register<T> {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Register(value) => Self(value, PhantomData))
    }
}

impl Parse for Address<Offset> {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::OffsetAddress => Self(PhantomData))
    }
}

impl Parse for Address<PreIndex> {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::PreIndexAddress => Self(PhantomData))
    }
}

impl Parse for Address<PostIndex> {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::PostIndexAddress => Self(PhantomData))
    }
}

impl Parse for Bang {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        match_buffer!(buffer: CIR::Bang => Self)
    }
}
