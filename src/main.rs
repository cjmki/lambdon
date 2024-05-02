use owo_colors::OwoColorize;
use clap::Parser;

const OWO: &str = r#"
██╗      █████╗ ███╗   ███╗██████╗  ██████╗ ███╗   ██╗
██║     ██╔══██╗████╗ ████║██╔══██╗██╔═══██╗████╗  ██║
██║     ███████║██╔████╔██║██║  ██║██║   ██║██╔██╗ ██║
██║     ██╔══██║██║╚██╔╝██║██║  ██║██║   ██║██║╚██╗██║
███████╗██║  ██║██║ ╚═╝ ██║██████╔╝╚██████╔╝██║ ╚████║
╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═════╝  ╚═════╝ ╚═╝  ╚═══╝
                                                          
"#;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    println!("\n\n\n\n\n{}", OWO.fg_rgb::<0x00, 0xFF, 0x00>().bold());
    println!("\n\n\n\n\n\n");

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}