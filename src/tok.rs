use std::str::{CharIndices, FromStr};

#[derive(Clone, Debug, PartialEq)]
pub enum Tok<'input> {
    Num(f64),
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
    Comma,
    Define,
    If,
    String(&'input str),
}

// This function takes a string as parameter and returns a vector of triples
// with the following structure: (start_position, token, end_position)
pub fn tokenize<'input>(s: &'input str) -> Vec<(usize, Tok<'input>, usize)> {
    let mut tokens = Vec::new();
    let mut char_indices = s.char_indices();
    let mut lookahead = char_indices.next();
    while let Some((ci, c)) = lookahead {
        // skip whitespace characters
        if !c.is_whitespace() {
            match c {
                '(' => tokens.push(Tok::LParen),
                '-' => tokens.push(Tok::Minus),
                ')' => tokens.push(Tok::RParen),
                '+' => tokens.push(Tok::Plus),
                '*' => tokens.push(Tok::Times),
                ',' => tokens.push(Tok::Comma),
                '/' => tokens.push(Tok::Div),
                '"' => {
                    let (ci, _) = char_indices.next().expect("Unclosed '\"'"); // consume opening '"'
                    let (slice_end, _) = take_while(ci, &mut char_indices, |c| c != '"');
                    lookahead = char_indices.next(); // consume closing '"'
                    tokens.push(Tok::String(&s[ci..slice_end]));
                    continue;
                }
                _ if c.is_digit(10) => {
                    let (slice_end, next) = take_while(ci, &mut char_indices, |c| c.is_digit(10));
                    lookahead = next;
                    tokens.push(Tok::Num(f64::from_str(&s[ci..slice_end]).unwrap()));
                    continue;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }

        // advance to next character by default
        lookahead = char_indices.next();
    }

    tokens
        .into_iter()
        .enumerate()
        .map(|(i, tok)| (i * 2, tok, i * 2 + 1))
        .collect()
}

fn take_while<F>(
    slice_start: usize,
    char_indices: &mut CharIndices,
    f: F,
) -> (usize, Option<(usize, char)>)
where
    F: Fn(char) -> bool,
{
    let mut slice_end = slice_start + 1;

    for (ci, c) in char_indices {
        if f(c) {
            slice_end = ci + 1;
        } else {
            return (ci, Some((ci, c)));
        }
    }

    (slice_end, None)
}

#[test]
fn tok1_test() {
    let source_code = "("; // should return (0, LParen, 1)
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Tok::LParen, 1)])
}

#[test]
fn tok2_test() {
    let source_code = "()";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Tok::LParen, 1), (2, Tok::RParen, 3)])
}

#[test]
fn tok3_test() {
    let source_code = "(1 2 3)";
    let tokens = tokenize(source_code);
    let expected = [
        (0, Tok::LParen, 1),
        (2, Tok::Num(1.0), 3),
        (4, Tok::Num(2.0), 5),
        (6, Tok::Num(3.0), 7),
        (8, Tok::RParen, 9),
    ];
    assert_eq!(tokens, expected)
}
