use async_trait::async_trait;
use axum::body::Bytes;
use log::info;
use pingora_core::{
    prelude::{background_service, HttpPeer, Server},
    Error,
};
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_load_balancing::{
    prelude::{RoundRobin, TcpHealthCheck},
    LoadBalancer,
};
use pingora_proxy::{ProxyHttp, Session};
use std::{sync::Arc, time::Duration};

struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora_core::Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();
        info!("Forwarding request to {}", upstream.addr.to_string());
        Ok(Box::from(HttpPeer::new(upstream, false, String::from(""))))
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        _upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora_core::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        _upstream_request
            .insert_header("x-proxy-from", "0.0.0.0:6193")
            .unwrap();

        Ok(())
    }

    async fn request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora_core::Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        if !_session.req_header().uri.path().starts_with("/health") {
            info!("Request not allowed");
            let _ = _session.respond_error(403).await;
            return Ok(true);
        }

        info!("Request allowed");
        Ok(false)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora_core::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        _upstream_response.insert_header("Name", "Jack").unwrap();
        Ok(())
    }

    /* The following code may contain inaccuracies and some parts are written based on assumptions */

    // async fn request_body_filter(
    //     &self,
    //     _session: &mut Session,
    //     _body: &mut Option<Bytes>,
    //     _end_of_stream: bool,
    //     _ctx: &mut Self::CTX,
    // ) -> pingora_core::Result<bool>
    // where
    //     Self::CTX: Send + Sync,
    // {
    //     todo!()
    // }

    // async fn response_body_filter(
    //     &self,
    //     _session: &mut Session,
    //     _body: &mut Option<Bytes>,
    //     _end_of_stream: bool,
    //     _ctx: &mut Self::CTX,
    // ) -> pingora_core::Result<Option<Duration>>
    // where
    //     Self::CTX: Send + Sync,
    // {
    //     todo!()
    // }

    // async fn logging(&self, _session: &mut Session, _e: Option<&Error>, _ctx: &mut Self::CTX)
    // where
    //     Self::CTX: Send + Sync,
    // {
    //     todo!()
    // }
}

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut upstreams = LoadBalancer::try_from_iter(["127.0.0.1:3000", "127.0.0.1:4000"]).unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(2));

    let background = background_service("health checker", upstreams);
    let upstreams = background.task();

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, LB(upstreams));

    proxy.add_tcp("0.0.0.0:6193"); // 127.0.0.1:6193/health

    server.add_service(proxy);
    server.run_forever();
}
