use crate::Encodable;

pub struct WordBuilder {
    cursor: u8,
    word: Word,
}

impl WordBuilder {
    pub const fn new() -> Self {
        Self {
            cursor: 32,
            word: Word::empty(),
        }
    }

    pub fn finish(self) -> Word {
        self.word
    }

    pub fn encode(mut self, enc: impl Encodable + std::fmt::Debug) -> Self {
        let size = enc.size();
        if size == 0 {
            // takes up no bits, don't even bother encoding
            return self;
        }

        let word = enc.encode();
        self.cursor -= size;
        self.word = self.word.with(word.get(), self.cursor);
        self
    }
}

impl Default for WordBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Word(pub(crate) u32);

impl Word {
    pub const fn empty() -> Self {
        Word(0)
    }

    pub const fn base(bits: u32) -> Self {
        Word(bits)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }

    #[must_use = "Word is modified using `with`"]
    pub const fn with(mut self, bits: u32, offset: u8) -> Self {
        self.0 |= bits << offset;
        self
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
