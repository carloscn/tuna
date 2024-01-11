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

    //let mut net_operator:NetOperator = NetOperator::new(&args.server, args.port, 1);

}

#[cfg(test)]
mod test_storage_manager {
    use std::io::{Error as IOError, ErrorKind};

    use super::*;

    #[test]
    fn test_is_dir() {
        let mut np:StorageManager = StorageManager::new(10000);
        let ret = np.is_dir_exist("/home/haochenwei").unwrap();
        assert_eq!(ret, true);

        let ret = np.is_dir_exist("/home/xxx").unwrap();
        assert_eq!(ret, false);

        let ret = np.is_dir_exist("");
        match ret {
            Ok(_) => panic!("Test failed!"),
            Err(e) => {
                match e.kind() {
                    ErrorKind::InvalidInput => {
                        assert!(true);
                    }
                    _ => panic!("test failed!"),
                }
            },
         }

        let ret = np.is_dir_exist("dd").unwrap();
        assert_eq!(ret, false);
    }

}

