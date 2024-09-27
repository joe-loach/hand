use ual_derive::UAL;

mod syntax;
mod lexer;

/// loop:
/// ADD r0, r1, #1
/// CMP r0, #100
/// BLT loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HAND {}

#[derive(UAL)]
#[ual = "ADD{S}{<c>} {<Rd>,} <Rn>, #<const>"]
pub struct ADD;