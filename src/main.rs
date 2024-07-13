use calculator::{Calculator, Error};

mod calculator;

fn main() -> Result<(), Error> {
    loop {
        let mut input = String::new();
        match io::io::stdin().read_line(&mut input) {
            Ol(_) => {
                let tokens = Calculator::parse(input);
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr = Calculator::expression(tokens?);
                if let Some(v) = Calculator::exvaluate(expr) {
                    println!("{}", v);
                }
            },
            Err(error) => println!("error: {}", error)
        }
    }
}
