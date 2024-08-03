mod lexer;
mod expr;

fn main() {
    let test_case = String::from(
        "fn fibonacchi = (n) {
            if (n <= 1) {
                return n;
            }
            return fibonacchi(n - 2) + fibonacchi(n - 1);
        }
        
        // run fibonacchi for numbers 0-19
        for (let i = 0; i < 20; i = i + 1) {
            print(fibonacchi(i));
        }");
    //let test_case = String::from("true false");

    let mut lexer = lexer::Lexer::new(test_case.clone());
    let tokens = lexer.tokenize_all();
    println!("{}", test_case);

    for token in &tokens {
        println!("{:?}", token);
    }
}
