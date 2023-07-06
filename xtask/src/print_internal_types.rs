use derive_codegen::Generation;
use gumdrop::Options;

#[derive(Options)]
pub(crate) struct SubOptions {
    #[options(help = "pretty json")]
    pretty: bool,
    #[options(help = "show help")]
    help: bool,
}

pub(crate) fn run(opts: SubOptions) {
    if opts.help {
        println!("{}", SubOptions::usage());
        return;
    }

    let selection = Generation::for_tag("derive-codegen-internal");
    println!(
        "{}",
        if opts.pretty {
            selection.to_input_json_pretty()
        } else {
            selection.to_input_json()
        }
    );
}
