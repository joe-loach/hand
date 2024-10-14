#![allow(dead_code)]

mod hand;
mod ual;

pub(crate) use hand::HANDCursor;
pub(crate) use ual::UALCursor;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Kind {
    Register            = 0b00001,
    RegisterList        = 0b00010,
    Condition           = 0b00011,
    Shift               = 0b00100,
    Number              = 0b00110,
    Bang                = 0b00111,
    OffsetAddress       = 0b01000,
    PreIndexAddress     = 0b01001,
    PostIndexAddress    = 0b01010,
    Ident               = 0b10000,
}

/// A packed struct representing a template
/// 
/// The [`Inner`] stuct contains two representations,
/// either a [`tag`](Inner::tag) or [`ident`](Inner::ident).
/// 
/// The [`tag`](Inner::tag) representation should look like this in memory:
/// ` 0 TAG 0 0 0 `
/// 
/// The [`ident`](Inner::ident) representation should look like this in memory:
/// ` 1 ASCII_CHAR `
/// 
/// Using the fact that ascii characters use only 7 bits,
/// we can use the MSB to toggle between the two representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Inner(u8);

impl Inner {
    const IDENT_MASK: u8 = 0b1000_0000;

    const fn ident(char: u8) -> Self {
        Self(Self::IDENT_MASK | char)
    }

    const fn tag(kind: Kind) -> Self {
        assert!(kind as u8 != Kind::Ident as u8);
        let bits = kind as u8;
        Self(bits << 3)
    }

    const fn decode(&self) -> (Kind, Option<u8>) {
        if self.0 & Self::IDENT_MASK != 0 {
            // decoding an ident
            let char = (!Self::IDENT_MASK) & self.0;
            (Kind::Ident, Some(char))
        } else {
            // normal tag
            let bits = self.0 >> 3;
            // SAFETY:
            // * size_of::<u8> == size_of::<Kind>
            // * not a ident tag
            // * created by casting to u8 and << 3
            // => shifted up 3 bits, to be created
            // => to retrive kind, do the opposite
            let kind = unsafe { std::mem::transmute::<u8, Kind>(bits) };
            (kind, None)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Template(Inner);

impl Template {
    pub const fn char(&self) -> Option<u8> {
        self.0.decode().1
    }

    pub const fn kind(&self) -> Kind {
        self.0.decode().0
    }

    pub const fn ident(char: u8) -> Self {
        Self(Inner::ident(char))
    }

    pub const fn register() -> Self {
        Self(Inner::tag(Kind::Register))
    }

    pub const fn register_list() -> Self {
        Self(Inner::tag(Kind::RegisterList))
    }

    pub const fn condition() -> Self {
        Self(Inner::tag(Kind::Condition))
    }

    pub const fn offset_address() -> Self {
        Self(Inner::tag(Kind::OffsetAddress))
    }

    pub const fn pre_index_address() -> Self {
        Self(Inner::tag(Kind::PreIndexAddress))
    }

    pub const fn post_index_address() -> Self {
        Self(Inner::tag(Kind::PostIndexAddress))
    }

    pub const fn shift() -> Self {
        Self(Inner::tag(Kind::Shift))
    }

    pub const fn number() -> Self {
        Self(Inner::tag(Kind::Number))
    }

    pub const fn bang() -> Self {
        Self(Inner::tag(Kind::Bang))
    }
}

impl std::fmt::Debug for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("Template");
        let tuple = if let Some(char) = self.char() {
            tuple.field(&std::char::from_u32(char as u32).unwrap())
        } else {
            tuple.field(&self.kind())
        };

        tuple.finish()
    }
}

#[test]
fn ident_roundtrip() {
    let t = Template::ident(b'A');
    assert_eq!(t.char(), Some(b'A'));
}

#[test]
fn tag_roundtrip() {
    assert_eq!(Template::register().kind(), Kind::Register);
    assert_eq!(Template::register_list().kind(), Kind::RegisterList);
    assert_eq!(Template::condition().kind(), Kind::Condition);
    assert_eq!(Template::offset_address().kind(), Kind::OffsetAddress);
    assert_eq!(Template::pre_index_address().kind(), Kind::PreIndexAddress);
    assert_eq!(Template::post_index_address().kind(), Kind::PostIndexAddress);
    assert_eq!(Template::shift().kind(), Kind::Shift);
    assert_eq!(Template::number().kind(), Kind::Number);
    assert_eq!(Template::bang().kind(), Kind::Bang);
}

#[test]
fn template_size() {
    assert_eq!(std::mem::size_of::<Template>(), 1);
}
