use gumdrop::Options;

#[derive(Options)]
pub(crate) struct SubOptions {
    // #[options(help = "pretty json")]
    // pretty: bool,
    #[options(help = "show help")]
    help: bool,
}

pub(crate) fn run(opts: SubOptions) {
    if opts.help {
        println!("{}", SubOptions::usage());
        return;
    }

    println!("Running dev");
}
