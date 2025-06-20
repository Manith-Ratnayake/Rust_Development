use std::time::Duration;
use std::io::{self, Write};
use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;


const MAX:u16 = 65535;

#[warn(dead_code)]
struct Arguments {
    flag : String,
    ipaddr : IpAddr,
    threads : u16,
}



impl Arguments {

    fn new(args: &[String]) -> Result<Arguments, &'static str>{

        if args.len() < 2 {
            return Err("Not Enough Arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments")
        }
        
        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads:4})

        }else {

            let flag = args[1].clone();

            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2{
                println!("Usage : -j to select how many threads you want \r\n-h or -help to show this message");
                return Err("help");

            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");

            } else if flag.contains("-j"){
                
                if args.len() != 4 {
                    return Err("Invalid syntax: use -j <threads> <ip>")
                }


                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IpAddr; must be a IPV4 or IPV6")
                };

                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s, 
                    Err(_) => return Err("Failed to parse the message")
                };

                return Ok(Arguments{threads, flag, ipaddr});

            } else {
                return Err("Invalid syntax");
            }


        }

        
    }
}



fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(1);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = channel();

    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();

    for v in out{
        println!("{} is open", v);
    }


    println!("program is over");

}


fn scan(tx: Sender<u16>, start_port: u16, addr:IpAddr, num_threads: u16) {

    let mut port: u16 = start_port + 1;
    while port <= MAX {

        let socket = (addr, port);
        let timeout = Duration::from_secs(1);
        
        if TcpStream::connect_timeout(&socket.into(), timeout).is_ok() {

                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
        }
                
        port += num_threads;


    }

}
