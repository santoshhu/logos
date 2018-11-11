extern crate logos;
#[macro_use] extern crate logos_derive;

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
pub enum Token {
    #[error]
    InvalidToken,

    #[end]
    EndOfProgram,

    #[regex = "[a-zA-Z$_][a-zA-Z0-9$_]*"]
    Identifier,

    #[regex = "[1-9][0-9]*"]
    Number,

    #[token = "priv"]
    Priv,

    #[token = "private"]
    Private,

    #[token = "primitive"]
    Primitive,

    #[token = "protected"]
    Protected,

    #[token = "protectee"]
    Protectee,

    #[token = "in"]
    In,

    #[token = "instanceof"]
    Instanceof,

    #[token = "."]
    Accessor,

    #[token = "..."]
    Ellipsis,

    #[token = "{"]
    BraceOpen,

    #[token = "}"]
    BraceClose,

    #[token = "+"]
    OpAddition,

    #[token = "++"]
    OpIncrement,

    #[token = "="]
    OpAssign,

    #[token = "=="]
    OpEquality,

    #[token = "==="]
    OpStrictEquality,

    #[token = "=>"]
    FatArrow,
}

use logos::Logos;
use std::ops::Range;

fn assert_lex(source: &str, tokens: &[(Token, &str, Range<usize>)]) {
    let mut lex = Token::lexer(source);

    for tuple in tokens {
        assert_eq!(&(lex.token, lex.slice(), lex.range()), tuple);

        lex.consume();
    }

    assert_eq!(lex.token, Token::EndOfProgram);
}

#[test]
fn empty() {
    let lex = Token::lexer("");

    assert_eq!(lex.token, Token::EndOfProgram);
    assert_eq!(lex.range(), 0..0);
}

#[test]
fn whitespace() {
    let lex = Token::lexer("     ");

    assert_eq!(lex.token, Token::EndOfProgram);
    assert_eq!(lex.range(), 5..5);
}

#[test]
fn operators() {
    assert_lex("=== == = => + ++", &[
        (Token::OpStrictEquality, "===", 0..3),
        (Token::OpEquality, "==", 4..6),
        (Token::OpAssign, "=", 7..8),
        (Token::FatArrow, "=>", 9..11),
        (Token::OpAddition, "+", 12..13),
        (Token::OpIncrement, "++", 14..16),
    ]);
}

#[test]
fn punctation() {
    assert_lex("{ . ... }", &[
        (Token::BraceOpen, "{", 0..1),
        (Token::Accessor, ".", 2..3),
        (Token::Ellipsis, "...", 4..7),
        (Token::BraceClose, "}", 8..9),
    ]);
}

#[test]
fn identifiers() {
    assert_lex("It was the year when they finally immanentized the Eschaton.", &[
        (Token::Identifier, "It", 0..2),
        (Token::Identifier, "was", 3..6),
        (Token::Identifier, "the", 7..10),
        (Token::Identifier, "year", 11..15),
        (Token::Identifier, "when", 16..20),
        (Token::Identifier, "they", 21..25),
        (Token::Identifier, "finally", 26..33),
        (Token::Identifier, "immanentized", 34..46),
        (Token::Identifier, "the", 47..50),
        (Token::Identifier, "Eschaton", 51..59),
        (Token::Accessor, ".", 59..60),
    ]);
}

#[test]
fn keywords() {
    assert_lex("foobar priv private primitive protected protectee in instanceof", &[
        (Token::Identifier, "foobar", 0..6),
        (Token::Priv, "priv", 7..11),
        (Token::Private, "private", 12..19),
        (Token::Primitive, "primitive", 20..29),
        (Token::Protected, "protected", 30..39),
        (Token::Protectee, "protectee", 40..49),
        (Token::In, "in", 50..52),
        (Token::Instanceof, "instanceof", 53..63),
    ]);
}

#[test]
fn keywords_mix_identifiers() {
    assert_lex("pri priv priva privb privat private privatee privateer", &[
        (Token::Identifier, "pri", 0..3),
        (Token::Priv, "priv", 4..8),
        (Token::Identifier, "priva", 9..14),
        (Token::Identifier, "privb", 15..20),
        (Token::Identifier, "privat", 21..27),
        (Token::Private, "private", 28..35),
        (Token::Identifier, "privatee", 36..44),
        (Token::Identifier, "privateer", 45..54),
    ]);
}

#[test]
fn numbers() {
    assert_lex("0 1 2 3 4 10 42 1337", &[
        (Token::InvalidToken, "0", 0..1),
        (Token::Number, "1", 2..3),
        (Token::Number, "2", 4..5),
        (Token::Number, "3", 6..7),
        (Token::Number, "4", 8..9),
        (Token::Number, "10", 10..12),
        (Token::Number, "42", 13..15),
        (Token::Number, "1337", 16..20),
    ]);
}

#[test]
fn invalid_tokens() {
    assert_lex("@-/!", &[
        (Token::InvalidToken, "@", 0..1),
        (Token::InvalidToken, "-", 1..2),
        (Token::InvalidToken, "/", 2..3),
        (Token::InvalidToken, "!", 3..4),
    ]);
}
