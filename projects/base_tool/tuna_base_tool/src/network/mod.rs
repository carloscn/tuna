use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

struct NetManager {
    server_ip: String,
    server_port: usize,
    server_timeout_s: usize,
    server_handler: TcpListener,
}

impl NetManager
{
    fn new(ip_address:&String, port:usize, timeout_s: usize) -> NetManager
    {
        let listen_addr: String = format!("{}:{}", ip_address, port);
        let handler:TcpListener = match TcpListener::bind(listen_addr) {
            Ok(h) => h,
            Err(error) => panic!("cannot bind ip {:?}:{:?}", ip_address, port),
        };
        NetManager {
            server_ip: ip_address.to_string(),
            server_port: port,
            server_handler: handler,
            server_timeout_s: timeout_s
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) -> Result<(), Error>
    {
        let mut buf = [0; 512];

        for _ in 0..1000 {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                return Ok(());
            }
            stream.write(&buf[..bytes_read])?;
            thread::sleep(time::Duration::from_secs(1 as u64));
        }

        Ok(())
    }

    fn free(&mut self) {

    }
}
pub struct NetOperator {
    net_manager: NetManager,
    server_ip: String,
    server_port: usize,
}

impl NetOperator
{
    pub fn new(ip_address:&String, port:usize, timeout_s: usize) -> NetOperator
    {

        let op = NetOperator {
            net_manager: NetManager::new(ip_address, port, timeout_s),
            server_ip: ip_address.to_string(),
            server_port: port,
        };

        println!("[INFO] Net listen to server {:?}:{:?}", ip_address, port);

        return op;
    }

    pub fn read_from_network(&self) ->  Result<usize, Error>
    {
        let mut read_len:usize = 0;
        Ok(read_len)
    }

    pub fn write_to_network(&self) -> Result<usize, Error>
    {
        let mut write_len:usize = 0;
        Ok(write_len)
    }

    pub fn free(&mut self)
    {
        self.net_manager.free();
        self.net_manager.server_ip = String::new();
        self.net_manager.server_port = 0;
        self.server_ip = String::new();
        self.server_port = 0;
    }
}