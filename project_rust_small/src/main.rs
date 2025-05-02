use clap::Parser;

fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Parser, Debug)]
#[clap(version)]
struct CliArgs {
    #[clap(short, long)]
    pub number: i32,
}

fn main() {
    let args = CliArgs::parse();
    let result = add_two(args.number);
    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
}