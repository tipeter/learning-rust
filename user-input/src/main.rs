fn user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    stdout().write_all(prompt.as_bytes());
    let _ = stdout().flush();

    let mut st = String::new();

    stdin()
        .read_line(&mut st)
        .expect("Did not enter a correct string!");
    st.trim().to_string()
}

fn parse_u32(st: &String) -> Result<u32, std::num::ParseIntError> {
    Ok(st.parse::<u32>()?)
}

fn original() -> i32 {
    let x = 5;
    let y = 6;
    x + y
}

fn desugared() -> i32 {
    match 5 {
        x => match 6 {
            y => x + y,
        },
    }
}

fn main() {
    let mut st = user_input("Please enter some text:");

    println!("The entered string: {:?}", st);
}
