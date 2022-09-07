use std::path::Path;
use std::ffi::OsStr;
use url::Url;
use once_cell::sync::OnceCell;

static IAM : OnceCell<String> = OnceCell::new();
static DSTSCHEME : OnceCell<String> = OnceCell::new();
static DSTDOMAIN : OnceCell<String> = OnceCell::new();
static DSTPATH : OnceCell<String> = OnceCell::new();
static mut DSTPORT : i32 = -1;
static DSTUSERID : OnceCell<String> = OnceCell::new();
static DSTPASSWD : OnceCell<String> = OnceCell::new();

static PROXYURL : OnceCell<String> = OnceCell::new();
static DEFPROXYURL : OnceCell<String> = OnceCell::new();
static PXYSCHEME : OnceCell<String> = OnceCell::new();
static PXYDOMAIN : OnceCell<String> = OnceCell::new();
static PXYPATH : OnceCell<String> = OnceCell::new();
static mut PXYPORT : i32 = -1;

pub fn usage()
{
    let iam = match crate::help::IAM.get()
    {
        Some(v) => v,
        None => todo!(),
    };
    println!("{} <targetURL> [{{-p|--proxy}} <proxyURL>] [-h|--help]", iam);
}

pub fn parse_argv(argc:usize, argv:Vec<String>) -> bool
{
    println!("[{}]{:?}", argc, argv);

    let cmdnm = match Path::new(&argv[0]).file_name()
    {
        Some(v) => v,
        None => {
            OsStr::new("Bye bye ...");
            return false;
        },
    };
    let _iam = match cmdnm.to_str()
    {
        Some(v) => crate::help::IAM.set(v.to_string()).unwrap(),
        None => todo!(),
    };

    if argc < 2
    {
        usage();
        false
    }
    else
    {
        crate::help::DEFPROXYURL.set("".to_string()).unwrap();

        let mut ax = 1;

        while ax < argc
        {
            if &argv[ax] == "--help" || &argv[ax] == "-h"
            {
                return false;
            }
            else if &argv[ax] == "--proxy"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("Missing Proxy URL");
                    return false;
                }
                let proxyurl = &argv[ax];
                println!("Proxy: {}", proxyurl);
                /***
                let proxy = Some(proxyurl).map(|a|
                    {
                        Url::parse(a).unwrap_or_else(|e|
                            {
                                println!("Illegal proxy URL: {}", a);
                                return false;
                            })
                    });
                ***/
                let proxy = match Url::parse(proxyurl)
                {
                    Ok(v) => v,
                    Err(e) =>
                    {
                        println!("Illegal proxy URL: {}",e);
                        return false;
                    }
                };
                crate::help::PROXYURL.set(proxyurl.to_string()).unwrap();

                let pxyscheme = proxy.scheme();
                println!("proxy scheme:{}", pxyscheme);
                let pxyhost = match proxy.host()
                {
                    Some(v) => v,
                    None =>
                    {
                        println!("Proxy Host error");
                        return false;
                    },
                };
                println!("proxy host:{}", pxyhost);
                let pxypath = proxy.path();
                println!("proxy path:{}", pxypath);
                let pxyport: i32 = match proxy.port()
                {
                    Some(v) => v.into(),
                    None => -1,
                };
                println!("proxy port:{:?}", pxyport);

                crate::help::PXYSCHEME.set(pxyscheme.to_string()).unwrap();
                crate::help::PXYDOMAIN.set(pxyhost.to_string()).unwrap();
                crate::help::PXYPATH.set(pxypath.to_string()).unwrap();
                unsafe {crate::help::PXYPORT = pxyport;}
            }
            else
            {
                let dsturl = match Url::parse(&argv[ax])
                {
                    Ok(v) => v,
                    Err(e) =>
                    {
                        println!("UrlParse error {}",e);
                        return false;
                    }
                };
                println!("ParseURL:{:?}", dsturl);
                let dstscheme = dsturl.scheme();
                println!("scheme:{}", dstscheme);
                let dsthost = match dsturl.host()
                {
                    Some(v) => v,
                    None =>
                    {
                        println!("UrlHost error");
                        return false;
                    },
                };
                println!("host:{}", dsthost);
                let dstpath = dsturl.path();
                println!("path:{}", dstpath);
                let dstport: i32 = match dsturl.port()
                {
                    Some(v) => v.into(),
                    None => -1,
                };
                println!("port:{:?}", dstport);
                let dstuserid = dsturl.username();
                println!("userid:{}", dstuserid);
                let dstpasswd = match dsturl.password()
                {
                    Some(v) => v,
                    None => "",
                };
                println!("password:{}", dstpasswd);

                crate::help::DSTSCHEME.set(dstscheme.to_string()).unwrap();
                crate::help::DSTDOMAIN.set(dsthost.to_string()).unwrap();
                crate::help::DSTPATH.set(dstpath.to_string()).unwrap();
                unsafe {crate::help::DSTPORT = dstport;}
                crate::help::DSTUSERID.set(dstuserid.to_string()).unwrap();
                crate::help::DSTPASSWD.set(dstpasswd.to_string()).unwrap();
            }
            ax += 1;
        }
        true
    }
}

pub mod get
{
    pub fn scheme() -> &'static String
    {
        let vl = match crate::help::DSTSCHEME.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn domain() -> &'static String
    {
        let vl = match crate::help::DSTDOMAIN.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn path() -> &'static String
    {
        let vl = match crate::help::DSTPATH.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn port() -> i32
    {
        unsafe {crate::help::DSTPORT}
    }
    pub fn userid() -> &'static String
    {
        let vl = match crate::help::DSTUSERID.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn passwd() -> &'static String
    {
        let vl = match crate::help::DSTPASSWD.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }

    pub fn proxyurl() -> &'static String
    {
        let vl = match crate::help::PROXYURL.get()
        {
            Some(v) => v,
            None => crate::help::DEFPROXYURL.get().unwrap(),
        };
        vl
    }
    pub fn proxyscheme() -> &'static String
    {
        let vl = match crate::help::PXYSCHEME.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn proxydomain() -> &'static String
    {
        let vl = match crate::help::PXYDOMAIN.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn proxypath() -> &'static String
    {
        let vl = match crate::help::PXYPATH.get()
        {
            Some(v) => v,
            None => todo!(),
        };
        vl
    }
    pub fn proxyport() -> i32
    {
        unsafe {crate::help::PXYPORT}
    }
}
