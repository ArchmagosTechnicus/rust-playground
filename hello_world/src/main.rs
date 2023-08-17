use ferris_says::say;
use std::io::stdout;

fn main() {
    let message = "Hello, world!";
    let stdout = stdout().lock();
    say(message, message.len(), stdout).unwrap();
}
