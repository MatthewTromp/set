use std::env;

mod normal;
mod llnormal;
mod projective;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--projective".to_string()) {
        projective::game_loop();
    } else {
        normal::game_loop();
    }
}
