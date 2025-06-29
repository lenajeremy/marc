use md_to_html::Lexer;

fn main() {
    let input = "\
        ## Hello World
# This is Jeremiah @#FFD%@## **hello**
# And this is a very important heading";
    let mut l = Lexer::from(input);
    println!("{}", l.src.escape_default());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
    println!("{:#?}", l.next_token());
}
