pub mod args {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// The year of the Advent of Code(so that path can be constructed).
        #[clap(short, long)]
        pub year: String,

        /// Flag to enable generation of graphviz images with the AST.
        #[clap(short, long)]
        pub day: String,
    }
}