use monty::PolicyNetwork;

fn main() {
    let mut args = std::env::args();
    args.next();
    let threads = args.next().unwrap().parse().unwrap();

    policy::train::<PolicyNetwork>(threads, "data/chess/converted-66b8dab93ae9310e136dd758-99.data".to_string(), 60, 25);
}
