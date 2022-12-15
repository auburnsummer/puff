use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).expect("A file should have been given as the first argument.");
    // dbg!(args);

    puff::run(file);
}
