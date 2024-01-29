use std::iter::Peekable;

#[derive(Clone, Debug)]
pub enum Node {
    Item(Value),
    List(Vec<Node>),
}

impl Node {
    pub fn as_item(self) -> Option<Value> {
        match self {
            Self::Item(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_list(self) -> Option<Vec<Self>> {
        match self {
            Self::List(items) => Some(items),
            _ => None,
        }
    }
}

type Value = String;

type Error = String;

pub fn from_str(string: &str) -> Result<Node, Error> {
    let tokens = parse_tokens(string)?;
    // print_tokens(&tokens);

    let mut tokens = tokens.into_iter().peekable();
    let nodes = from_tokens(&mut tokens, 0)?;
    // println!("{:#?}", nodes);

    let mut nodes = nodes.into_iter();
    let first_node = nodes.next().unwrap();

    if nodes.next().is_some() {
        return Err(format!("unexpected node. only one top level node allowed."));
    }

    Ok(first_node)
}

fn from_tokens<I>(tokens: &mut Peekable<I>, recursion: usize) -> Result<Vec<Node>, Error>
where
    I: Iterator<Item = Token>,
{
    debug_assert!(recursion < 200, "max recursion");

    let mut nodes = Vec::new();

    while let Some(token_peek) = tokens.peek() {
        // println!("{:?}", token_peek);

        match token_peek {
            Token::BracketOpen => {
                let _ = tokens.next().unwrap();
                let new_nodes = from_tokens(tokens, recursion + 1)?;
                // println!("{:?}", new_nodes);
                nodes.push(Node::List(new_nodes));
            }
            Token::BracketClose => {
                let _ = tokens.next().unwrap();
                if recursion == 0 {
                    return Err(format!("unexpected closing bracket."));
                }
                return Ok(nodes);
            }
            Token::ItemText(_) => {
                let token = tokens.next().map(Token::as_item_text).flatten().unwrap();
                nodes.push(Node::Item(token));
            }
        }
    }

    if recursion > 0 {
        return Err(format!(
            "unexpected end of string. expected closing bracket"
        ));
    }

    Ok(nodes)
}

#[derive(Debug)]
enum Token {
    BracketOpen,
    BracketClose,
    ItemText(String),
}

impl Token {
    pub fn as_item_text(self) -> Option<String> {
        match self {
            Self::ItemText(text) => Some(text),
            _ => None,
        }
    }
}
//
// fn print_tokens(tokens: &[Token]) {
//     let mut indent = 0;
//     for token in tokens {
//         match token {
//             Token::BracketOpen => {
//                 print!("{}", "  ".repeat(indent * 2));
//                 println!("[");
//                 indent += 1;
//             }
//             Token::BracketClose => {
//                 if indent > 0 {
//                     indent -= 1;
//                 }
//                 print!("{}", "  ".repeat(indent * 2));
//                 println!("]")
//             }
//             Token::ItemText(text) => {
//                 print!("{}", "  ".repeat(indent * 2));
//                 println!("<{}>", text)
//             }
//         }
//     }
// }

fn parse_tokens(string: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut current_text = String::new();

    let mut is_quoted = false;
    let mut is_escaped = false;

    //TODO: change `current_text` to `Option<String>` ?
    // `[    ]` should resolve to no items
    // `[ "" ]` should resolve to one item, an empty string

    //TODO: remove duplicate code
    // `tokens.push(Token::ItemText(current_text))`

    for ch in string.chars() {
        if is_escaped {
            let escaped = match ch {
                // cannot use linebreaks in an item (ever)
                '\n' => return Err(format!("unexpected end of line")),
                // escaped whitespace
                _ if ch.is_whitespace() => ch,
                // standard escape characters
                'n' => '\n',
                't' => '\t',
                '"' => ch,
                // only escape brackets if NOT in quotes
                '[' | ']' => {
                    if is_quoted {
                        return Err(format!("unnecessary character escape in quotes `{}`", ch));
                    }
                    ch
                }
                _ => return Err(format!("unknown character escape `{}`", ch)),
            };
            current_text.push(escaped);
            is_escaped = false;
            continue;
        }

        match ch {
            '\\' => is_escaped = true,
            '"' => is_quoted ^= true, // flip boolean

            _ if is_quoted => {
                if ch == '\n' {
                    return Err(format!("unexpected end of line"));
                }
                current_text.push(ch);
            }

            '[' => {
                if !current_text.is_empty() {
                    tokens.push(Token::ItemText(current_text));
                    current_text = String::new();
                }
                tokens.push(Token::BracketOpen);
            }
            ']' => {
                if !current_text.is_empty() {
                    tokens.push(Token::ItemText(current_text));
                    current_text = String::new();
                }
                tokens.push(Token::BracketClose);
            }

            _ if ch.is_whitespace() => {
                if !current_text.is_empty() {
                    tokens.push(Token::ItemText(current_text));
                    current_text = String::new();
                }
            }

            _ => current_text.push(ch),
        }
    }

    if !current_text.is_empty() {
        tokens.push(Token::ItemText(current_text));
    }

    Ok(tokens)
}
