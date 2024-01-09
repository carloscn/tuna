use clap::{Parser, parser};
mod network;
mod storage;
use network::NetOperator;
use storage::StorageManager;

// tuna_base_tool --server "10.10.127.59" --port 11225 --path "${HOME}/adc/data" --fft-enable

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ip server address (local host)
    #[arg(short, long)]
    server: String,

    /// port number
    #[arg(short, long, default_value_t = 11225)]
    port: usize,

    /// data storage path
    #[arg(long)]
    path: String,

    /// fft-enable
    #[arg(short, long)]
    fft_enable: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            server: "127.0.0.1".to_string(),
            port: 11225,
            path: "/home/hello".to_string(),
            fft_enable: false,
            count: 1
        }
    }
}

fn main() {

    let args = Args::parse();

    println!("Hello {}", args.server);
    println!("Hello {}", args.port);
    println!("Hello {}", args.path);
    println!("Hello {}", args.fft_enable);

    let mut net_operator:NetOperator = NetOperator::new(&args.server, args.port, 1);

    net_operator.free();
}

