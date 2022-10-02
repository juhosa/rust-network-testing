use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

#[derive(Debug)]
struct HttpPacket {
    // headers: Headers,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpPacket {
    fn new(data: String) -> Self {
        let mut hs: Vec<&str> = Vec::new();
        let mut body: Vec<&str> = Vec::new();

        let mut still_headers = true;

        for r in data.split("\r\n") {
            // println!("r: {r}");
            if r.len() == 0 {
                still_headers = false;
                continue;
            }

            if still_headers {
                hs.push(r);
            } else {
                body.push(r);
            }
        }

        // println!("hs: {:?}", hs);

        let mut headers: HashMap<String, String> = HashMap::new();

        // First version, works
        // for h in hs {
        //     match h.split_once(":") {
        //         Some(things) => {
        //             println!("name: {}, value: {}", things.0, things.1);
        //             headers.insert(
        //                 things.0.trim().to_lowercase().into(),
        //                 things.1.trim().into(),
        //             );
        //         }
        //         None => {
        //             println!("h is {}", h);
        //             headers.insert("status".into(), h.into());
        //         }
        //     }
        // }

        // Does the same as above
        let mut hs = hs.iter();
        // the first is PROTOCOL STATUS
        let ps = hs.next().unwrap();
        headers.insert("status".into(), ps.to_string());
        // rest are headers
        for h in hs {
            let h = h.split_once(":").unwrap();
            headers.insert(
                h.0.to_lowercase().to_string(),
                h.1.trim().to_lowercase().to_string(),
            );
        }

        // println!("body: {:?}", body);
        let body: String = body.concat();
        HttpPacket { headers, body }
    }
}

#[derive(Debug)]
struct Headers {
    date: String,
    content_type: String,
}

// HttpPacket.Headers.ContentType
// HttpPacket.headers['content-type]

fn main() {
    let domainname = "httpbin.org";
    // let uri = "54.236.79.58:80";
    // println!("uri: {}", uri);

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(domainname).unwrap();

    let uri = response.iter().next().expect("no address!");
    let mut ipport = uri.to_string();
    ipport.push_str(":80");
    println!("domain: {}, ip: {}", domainname, ipport);

    if let Ok(mut stream) = TcpStream::connect(ipport) {
        println!(
            "connected to uri {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        let message = "GET /ip HTTP/1.1\r\nHost: httpbin.org\r\n\r\n";
        let _ = stream.write_all(message.as_bytes());
        println!("sent:");
        println!("{}", message);

        let mut buffer = [0; 1024];

        let len = stream.read(&mut buffer).unwrap();

        println!("stream len: {}", len);

        let data = String::from_utf8_lossy(&buffer[..len]);

        println!("received data:");
        // println!("{:?}", data);

        for d in data.split("\r\n") {
            println!("{}", d);
        }

        let hp = HttpPacket::new(data.to_string());
        println!("{:#?}", hp);
        // println!("{:#?}", hp.headers);
    } else {
        println!("error connectin to {}", uri);
    }
}
