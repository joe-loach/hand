pub struct Cursor<'t> {
    chars: std::iter::Peekable<std::str::CharIndices<'t>>,
    last_idx: usize,
}

impl<'t> Cursor<'t> {
    #[inline]
    pub(crate) fn new(text: &'t str) -> Self {
        assert!(!text.is_empty());

        Cursor {
            chars: text.char_indices().peekable(),
            last_idx: 0_usize,
        }
    }

    #[inline]
    pub(crate) fn finish(self) -> usize {
        self.last_idx + 1
    }

    #[inline]
    pub fn eat(&mut self) -> Option<char> {
        self.eat_inner().map(|(_pos, char)| char)
    }

    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.peek_inner().map(|(_pos, char)| char)
    }

    pub fn eat_while(&mut self, pred: impl Fn(char) -> bool) -> usize {
        let mut consumed = 0_usize;

        while let Some(c) = self.peek() {
            if pred(c) {
                consumed += 1;
                self.eat();
            } else {
                break;
            }
        }

        consumed
    }

    fn eat_inner(&mut self) -> Option<(usize, char)> {
        let (pos, c) = self.chars.next()?;
        self.last_idx = pos;
        Some((pos, c))
    }

    fn peek_inner(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }
}
