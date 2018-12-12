use std::env::{args, Args};

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidParameter(String),
    InvalidInteger(String),
}

fn get_next_arg(args: &mut Args) -> Result<String, ParseError> {
    match args.next() {
        None => Err(ParseError::TooFewArgs),
        Some(s) => Ok(s),
    }
}

fn required_no_args(args: &mut Args) -> Result<(), ParseError> {
    match args.next() {
        None => Ok(()),
        Some(s) => Err(ParseError::TooManyArgs),
    }
}

fn get_token_value_pair(parstr: &String) -> Result<(&str, u32), ParseError> {
    let pieces: Vec<&str> = parstr.split("=").collect();
    if pieces.len() == 2 {
        if let Ok(value) = pieces[1].parse::<u32>() {
            Ok((pieces[0], value))
        } else {
            Err(ParseError::InvalidInteger(pieces[1].to_string()))
        }
    } else {
        Err(ParseError::InvalidParameter(parstr.clone()))
    }
}

fn get_token_value_pair_2(parstr: &String) -> Result<(&str, u32), ParseError> {
    let mut str_it = parstr.split("=");
    if let Some(token) = str_it.next() {
        if let Some(value_st) = str_it.next() {
            if let Ok(value) = value_st.parse::<u32>() {
                if str_it.next() == None {
                    return Ok((token, value))
                }
            }
        }
    }
    Err(ParseError::TooManyArgs)
}

fn parse_args(args: Args) -> Result<Frame, ParseError> {
    match args.len() {
        1...2 => return Err(ParseError::TooFewArgs),
        3 => {
            let mut width = None;
            let mut height = None;

            for arg in args.skip(1) {
                let (token_st, value) = {
                    let (token_st, value_st) = arg.split_at({
                        if let Some(pos) = arg.find('=') {
                            if (pos + 1) < arg.len() {
                                pos + 1
                            } else {
                                return Err(ParseError::InvalidParameter(arg.clone()));
                            }
                        } else {
                            return Err(ParseError::InvalidParameter(arg.clone()));
                        }
                    });

                    if let Ok(value) = value_st.parse::<u32>() {
                        (token_st, value)
                    } else {
                        return Err(ParseError::InvalidInteger(value_st.to_string()));
                    }
                };

                match token_st {
                    "width=" => {
                        width = Some(value);
                    }
                    "height=" => {
                        height = Some(value);
                    }
                    _ => {
                        return Err(ParseError::InvalidParameter(token_st.to_string()));
                    }
                }
            }

            match (width, height) {
                (Some(..), Some(..)) => Ok(Frame {
                    width: width.unwrap(),
                    height: height.unwrap(),
                }),
                _ => Err(ParseError::TooFewArgs),
            }
        }
        _ => return Err(ParseError::TooManyArgs),
    }
}

fn main() {
    let mut st = "width=34 height=4d2".to_string();

    println!("{:?}", parse_args(std::env::args()));

    let x = ParseError::InvalidInteger("Error van!".to_string());

    println!("{:?}", x);
    if let ParseError::InvalidInteger(a) = x {
        println!("{}", a);
    }
}
