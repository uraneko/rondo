mod build;
mod command;
mod generate;
mod parse;

fn main() {
    let mut args = std::env::args();
    args.next();

    let cmd = command::Command::new(args).unwrap();
    cmd.cmd().unwrap();
}
