use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::io::BufWriter;
use std::io::Write;

pub fn connect(dsthost:&str, dstport:u16) -> Result<TcpStream, String>
{
    let dst_host_port = format!("{}:{}", dsthost, dstport);
    let mut dstaddr = dst_host_port.to_socket_addrs().unwrap();

    if let Some(addr) = dstaddr.find(|x| (*x).is_ipv4())
    {
        match TcpStream::connect(addr)
        {
            Ok(stream) =>
            {
                println!("Connecct success. : {}", dst_host_port);
                Ok(stream)
            },
            Err(er) =>
            {
                let xer = format!("Connecct faile. {} : {}", er, dst_host_port);
                eprintln!("{}",xer);
                Err(xer)
            }
        }
    }
    else
    {
        let xer = format!("Illegal destination : {}", dst_host_port);
        Err(xer)
    }
}

struct Auth {
    method: &'static str,
    credentials: String,
}
struct ProxyAuth {
    method: &'static str,
    credentials: String,
}
impl Auth {
    fn basic(username: &str, password: &str) -> Auth
    {
        let credentials = httpget::base64::encode(format!("{}:{}", username, password));
        Auth{method: "BASIC", credentials}
    }
    fn as_tag(&self) -> String
    {
        format!("Authorization: {} {}\r\n", self.method, self.credentials)
    }
}
impl ProxyAuth {
    fn basic(username: &str, password: &str) -> ProxyAuth
    {
        let credentials = httpget::base64::encode(format!("{}:{}", username, password));
        ProxyAuth{method: "BASIC", credentials}
    }
    fn as_tag(&self) -> String
    {
        format!("Proxy-Authorization: {} {}\r\n", self.method, self.credentials)
    }
}

fn get_auth(usrid: &str, passwd: &str) -> Option<Auth>
{
    if 0 < usrid.len()
    {Some(Auth::basic(usrid, passwd))}
    else
    {None}
}
fn get_proxy_auth(usrid: &str, passwd: &str) -> Option<ProxyAuth>
{
    if 0 < usrid.len()
    {Some(ProxyAuth::basic(usrid, passwd))}
    else
    {None}
}
pub fn function(stream:TcpStream, userid: &str, passwd: &str, host: &str, path: &str)
{

    let auth = match get_auth(userid, passwd)
    {
        Some(v) => v.as_tag(),
        None => "".to_string(),
    };
    println!("auth: {}", auth);
    /***
    .map(|auth|
        {
            if auth == None
            {""}
            else
            {auth.as_tag();}
        });
    ***/
    let sndbuf = format!("GET {} HTTP/1.1\r\n{}Host: {}\r\nUser-Agent: webget/0.1\r\nAccept: */*\r\nConnection: close\r\n\r\n", path, auth, host);
    //let sndbuf = format!("GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: webget/0.1\r\nAccept: */*\r\nConnection: close\r\n\r\n", path, host);
    let mut rcvbuf = String::new();

    let mut sender = BufWriter::new(&stream);
    let mut receiver = BufReader::new(&stream);

    let sact = sender.write(sndbuf.as_bytes()).expect("Send error!!!");
    println!("Snd:[{}]\n{}", sact, sndbuf);
    let _rslt = sender.flush();

    let mut rttl = 0;
    let mut ract = receiver.read_line(&mut rcvbuf).expect("Receive error!!!");
    print!("Rcv:[{}]",ract);
    while ract != 0
    {
        rttl += ract;
        ract = receiver.read_line(&mut rcvbuf).expect("Receive error!!!");
        print!("[{}]",ract);
    }
    println!("\nRcv:[{}]\n{}", rttl, rcvbuf);
}
