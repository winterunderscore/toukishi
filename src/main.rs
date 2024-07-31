mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new(String::from(
            "fn fibonacchi = (n) {
                if (n <= 1) {
                    return n;
                }
                return fibonacchi(n - 2) + fibonacchi(n - 1);
            }
            
            for (let i = 0; i < 20; i = i + 1) {
                print(fibonacchi(i));
            }
            "
            ));
    let tokens = lexer.tokenize_all();
    print!("{:?}", tokens);
}
