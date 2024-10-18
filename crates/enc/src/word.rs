#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Word(pub(crate) u32);

impl Word {
    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl std::ops::Deref for Word {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary = format!("{:032b}", self.0);
        let binary = binary.bytes().collect::<Vec<_>>();
        let mut nibbles = binary.as_slice().chunks_exact(4);

        write!(f, "0b")?;
        for (i, n) in nibbles.by_ref().enumerate() {
            let [a, b, c, d] = n else { unreachable!() };
            fn to_char(x: u8) -> char {
                char::from_u32(x as u32).unwrap()
            }
            write!(
                f,
                "{}{}{}{}",
                to_char(*a),
                to_char(*b),
                to_char(*c),
                to_char(*d)
            )?;
            if i != 7 {
                write!(f, "_")?;
            }
        }

        assert!(nibbles.remainder().is_empty());
        Ok(())
    }
}

impl PartialEq<u32> for Word {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Word> for u32 {
    fn eq(&self, other: &Word) -> bool {
        *other == *self
    }
}
