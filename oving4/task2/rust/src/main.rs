fn main() {
    let input = "This is a <test> & example.";
    println!("{}", format_string(input));
}

fn format_string(input:&str) -> String {
    input.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt")
}