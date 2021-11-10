use std::io;

use futures::SinkExt;

use loony::codec::{BytesCodec, Framed};
use loony::connect::Connect;
use loony::rt::net::TcpStream;
use loony::server::test_server;
use loony::service::{fn_service, Service, ServiceFactory};
use loony::util::Bytes;

#[cfg(feature = "openssl")]
#[loony::test]
async fn test_string() {
    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let conn = loony::connect::Connector::default();
    let addr = format!("localhost:{}", srv.addr().port());
    let con = conn.call(addr.into()).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());
}

#[cfg(feature = "rustls")]
#[loony::test]
async fn test_rustls_string() {
    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let conn = loony::connect::Connector::default();
    let addr = format!("localhost:{}", srv.addr().port());
    let con = conn.call(addr.into()).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());
}

#[loony::test]
async fn test_static_str() {
    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let conn = loony::connect::Connector::new();

    let con = conn.call(Connect::with("10", srv.addr())).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());

    let connect = Connect::new("127.0.0.1".to_owned());
    let conn = loony::connect::Connector::new();
    let con = conn.call(connect).await;
    assert!(con.is_err());
}

#[loony::test]
async fn test_new_service() {
    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let factory = loony::connect::Connector::new();
    let conn = factory.new_service(()).await.unwrap();
    let con = conn.call(Connect::with("10", srv.addr())).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());
}

#[cfg(feature = "openssl")]
#[loony::test]
async fn test_uri() {
    use std::convert::TryFrom;

    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let conn = loony::connect::Connector::default();
    let addr =
        loony::http::Uri::try_from(format!("https://localhost:{}", srv.addr().port()))
            .unwrap();
    let con = conn.call(addr.into()).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());
}

#[cfg(feature = "rustls")]
#[loony::test]
async fn test_rustls_uri() {
    use std::convert::TryFrom;

    let srv = test_server(|| {
        fn_service(|io: TcpStream| async {
            let mut framed = Framed::new(io, BytesCodec);
            framed.send(Bytes::from_static(b"test")).await.unwrap();
            Ok::<_, io::Error>(())
        })
    });

    let conn = loony::connect::Connector::default();
    let addr =
        loony::http::Uri::try_from(format!("https://localhost:{}", srv.addr().port()))
            .unwrap();
    let con = conn.call(addr.into()).await.unwrap();
    assert_eq!(con.peer_addr().unwrap(), srv.addr());
}
