use crate::CIR;

pub trait Structured: Sized {
    fn parse(cir: &[CIR]) -> Self;
}

pub struct Label;
pub struct Condition;
pub struct RegisterList;
pub struct Number<const BITS: u8>;
pub struct Shift;

pub struct D;
pub struct N;
pub struct M;
pub struct R;
pub struct S;
pub struct T;
pub struct Register<T>(T);

pub struct Offset;
pub struct PreIndex;
pub struct PostIndex;
pub struct Address<T>(T);

pub struct Bang;


impl Structured for crate::Condition {
    fn parse(cir: &[CIR]) -> Self {
        todo!()
        
    }
}


