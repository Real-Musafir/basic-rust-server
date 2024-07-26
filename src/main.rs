use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;


fn main() {
    let server = Server::new("127.0.0.1:8080".to_string()); //Server here is struct. New is associated function in impl
    server.run(); //This run method probably never return. It will loop forever waiting for your connections on somebody's speed socket

}
