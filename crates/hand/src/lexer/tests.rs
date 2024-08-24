use super::lex;
use crate::syntax::SyntaxKind::{self, *};

fn tokens(s: &str) -> Vec<SyntaxKind> {
    lex(s.into()).map(|i| i.tok).collect::<Vec<_>>()
}

#[test]
fn example() {
    tokens(
        r#"
start:
        MOV      r0, #10        ; Set up parameters
        MOV      r1, #3
        ADD      r0, r0, r1     ; r0 = r0 + r1
stop:
        MOV      r0, #0x18      ; angel_SWIreason_ReportException
        LDR      r1, =0x20026   ; ADP_Stopped_ApplicationExit
        SVC      #0x123456      ; ARM semihosting (formerly SWI)
        END                     ; Mark end of file"#,
    );
}

#[test]
fn chars() {
    assert_eq!(
        &tokens("{}[],#+-=!:"),
        &[
            OpenCurly,
            CloseCurly,
            OpenSquare,
            CloseSquare,
            Comma,
            Hash,
            Plus,
            Minus,
            Equals,
            Bang,
            Colon
        ]
    );
}

#[test]
fn numbers() {
    assert_eq!(&tokens(r"0"), &[Decimal]);
    assert_eq!(&tokens(r"1 9"), &[Decimal, Whitespace, Decimal]);
    assert_eq!(&tokens(r"0b01010"), &[Binary]);
    assert_eq!(&tokens(r"0o7610"), &[Octal]);
    assert_eq!(&tokens(r"0xFACE"), &[Hex]);
}

#[test]
fn comments() {
    assert_eq!(&tokens(r"; Comment"), &[Comment]);

    assert_eq!(&tokens(r"// C-Comment"), &[Comment]);

    assert_eq!(&tokens(r"/* Multiline-C-Comment */"), &[Comment]);

    assert_eq!(
        &tokens(r"/* Nested /* Multiline-C-Comment */ */ /* Another */"),
        &[Comment, Whitespace, Comment]
    );
}
