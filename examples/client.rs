use std::env;
use std::net::SocketAddr;
use http_body_util::Empty;
use hyper::Request;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

fn main() {
    let args: Vec<String> = env::args().collect();

    tokio::fstack_init(args.len(), args);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();


    let url = String::from("http://ifconfig.me/ip");
    let url = url.parse::<hyper::Uri>().expect("failed to parse URL");
    if url.scheme_str() != Some("http") {
        eprintln!("This example only works with 'http' URLs.");
    }
    let local_ip = String::from("172.31.2.192:0");
    let local_addr: SocketAddr = local_ip.parse().unwrap();

    // let connector = HttpConnector::new();
    let mut connector = HttpConnector::new();
    // connector.set_interface() // bind to device
    connector.set_local_address(Some(local_addr.ip())); // bind to address
    // connector.set_local_addresses() // bind to ipv4 and ipv6 addresses

    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);

    let req = Request::builder()
        .uri(url)
        .body(Empty::<bytes::Bytes>::new())
        .expect("failed to build request");


    rt.block_on(async {
        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
            let resp = client.request(req).await.expect("failed to fetch URL");
            eprintln!("{:?} {:?}", resp.version(), resp.status());
            eprintln!("{:#?}", resp.headers());
        }).await;
    });
}
