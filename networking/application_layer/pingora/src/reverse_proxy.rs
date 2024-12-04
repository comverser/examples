use async_trait::async_trait;
use pingora_core::prelude::{HttpPeer, Server};
use pingora_proxy::{ProxyHttp, Session};

struct ReverseProxy;

#[async_trait]
impl ProxyHttp for ReverseProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora_core::Result<Box<HttpPeer>> {
        Ok(Box::from(HttpPeer::new(
            String::from("127.0.0.1:3000"),
            false,
            String::from(""),
        )))
    }
}

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, ReverseProxy);

    proxy.add_tcp("0.0.0.0:6193"); // 127.0.0.1:6193/health

    server.add_service(proxy);
    server.run_forever();
}
