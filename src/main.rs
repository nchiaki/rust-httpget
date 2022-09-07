use std::env;

mod help;
mod tcpip;

use crate::help::get;

/***
macro_rules! exit_with_message
{
    ($($arg:tt)*) =>
    {
        eprintln!($($arg)*);
        std::process::exit(-1);
    }
}
***/

fn main() {
    let mut dsthost = "localhost";
    let mut dstpath = "/index.html";
    let mut dstport = 80;
    let mut proxyurl = "";

    println!("Hello, world!");

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    match help::parse_argv(argc, argv)
    {
        false => help::usage(),
        true =>
        {
            let prmscheme = get::scheme();
            let prmdomain = get::domain();
            let prmpath = get::path();
            let prmport = get::port();
            let prmuserid = get::userid();
            let prmpasswd = get::passwd();
            let prmproxyurl = get::proxyurl();

            dsthost = prmdomain;
            if prmpath.len() != 0
            {dstpath = prmpath;}
            if prmport < 0
            {
                if prmscheme == "http"
                {dstport = 80;}
                else if prmscheme == "https"
                {dstport = 443;}
                else
                {
                    help::usage();
                    return;
                }
            }
            else
            {dstport = prmport;}

            if prmproxyurl.len() != 0
            {proxyurl = prmproxyurl;}

            let getpath = format!("{}://{}:{}{}", prmscheme, prmdomain, dstport, prmpath);

            if 0 < prmuserid.len()
            {
                print!("{}", prmuserid);
                if 0 < prmpasswd.len()
                {print!(":{}", prmpasswd);}
                println!(" {}", getpath);
            }
            else
            {println!("{}", getpath);}

            if 0 < proxyurl.len()
            {// Via Proxy
                let proxyurl = get::proxyurl();
                let proxyscheme = get::proxyscheme();
                let proxydomain = get::proxydomain();
                let proxypath =   get::proxypath();
                let mut proxyport =   get::proxyport();

                if proxyport < 0
                {
                    if proxyscheme == "http"
                    {proxyport = 80;}
                    else if proxyscheme == "https"
                    {proxyport = 443;}
                    else
                    {
                        help::usage();
                        return;
                    }
                }
                let getproxypath = format!("{}://{}:{}{}", proxyscheme, proxydomain, proxyport, proxypath);
                println!("{}", getproxypath);

                // Connect Proxy Server
                let proxyStream = tcpip::connect(proxydomain, proxyport.try_into().unwrap()).unwrap();
                println!("{:?}", proxyStream);

                // Request Proxy server
                tcpip::function(proxyStream, &prmuserid, &prmpasswd, &dsthost, &getpath);
            }
            else
            {
                let stream = tcpip::connect(dsthost, dstport.try_into().unwrap()).unwrap();
                println!("{:?}", stream);
                tcpip::function(stream, &prmuserid, &prmpasswd, &dsthost, &dstpath);
            }
        },
    }
}
