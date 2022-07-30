use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    And,
    Or,
    Not,
    Var(String),
    Imp,
    LBracket,
    RBracket,
    True,
    False,
}

pub struct Tokeniser<'a> {
    chars: Peekable<Chars<'a>>,
    // text:
}

impl<'a> Iterator for Tokeniser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let current = loop {
            match self.chars.next() {
                Some(c) => match c {
                    c if c.is_whitespace() => continue,
                    c => break Some(c),
                },
                None => break None,
            };
        };

        match current {
            Some(c) => match c {
                '(' => Some(LBracket),
                ')' => Some(RBracket),
                'T' | 't' => Some(True),
                'F' | 'f' => Some(False),

                c if c.is_alphabetic() => {
                    let mut text = String::new();
                    text.push(c);

                    loop {
                        match self.chars.peek() {
                            Some(n) => {
                                if n.is_alphabetic() && !is_literal(c) {
                                    text.push(self.chars.next().unwrap());
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }

                    match text.as_str() {
                        "and" => Some(And),
                        "or" => Some(Or),
                        "not" => Some(Not),
                        "imp" => Some(Imp),
                        t => Some(Var(t.to_string())),
                    }
                }
                _ => panic!("Unsupported character"),
            },
            None => None,
        }
    }
}

fn is_literal(c: char) -> bool {
    c == '(' || c == ')' || c == 't' || c == 'T' || c == 'f' || c == 'F'
}

impl<'a> Tokeniser<'a> {
    pub fn new(text: &'a str) -> Peekable<Tokeniser> {
        Tokeniser {
            chars: text.chars().peekable(),
        }
        .peekable()
    }
}

#[test]
// fn test_tokeniser() {
//     let string = "if you run into errors just delete all your code";

//     let tokeniser = Tokeniser::new(&string);

//     for token in tokeniser {
//         println!("Token = {:?}", token);
//     }
// }
fn test_with_actual_tokens() {
    use Token::*;
    let string = "(t and B) imp (C or f)";
    let expected = [
        LBracket,
        True,
        And,
        Var("B".to_string()),
        RBracket,
        Imp,
        LBracket,
        Var("C".to_string()),
        Or,
        False,
        RBracket,
    ]
    .into_iter();

    let tokeniser = Tokeniser::new(&string);

    for (actual, expected) in Iterator::zip(tokeniser, expected) {
        // println!("actual : {:?}. expected : {:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}
