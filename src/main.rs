use crate::lexer::Lexer;

mod lexer;

fn main() {
    let lexer = Lexer::default();
    let tokens = lexer.tokenize(r#"
        def main() {
            a: int = 3;


            int a;
            if x == 5{
                a = 3
            } else {
                b = 10;
                a = 10;
            }
        }
    "#);
}
