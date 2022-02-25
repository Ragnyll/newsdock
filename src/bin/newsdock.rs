use newsdock::conf;
use newsdock::conf::CmdType;

fn main() {
    let conf = conf::build_conf();

    match conf.cmd_type {
        CmdType::Dl => {
            println!("downloading")
        },
        CmdType::Open => {
            println!("Opening")
        },
        CmdType::Clean => {
            println!("Cleaning")
        }
    }
}
