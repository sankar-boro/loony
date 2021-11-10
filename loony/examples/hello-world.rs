use std::{env, io};

use futures::future;
use log::info;
use loony::http::header::HeaderValue;
use loony::http::{HttpService, Response};
use loony::{server::Server, time::Seconds};

#[loony::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "hello_world=info");
    env_logger::init();

    Server::build()
        .bind("hello-world", "127.0.0.1:8080", || {
            HttpService::build()
                .client_timeout(Seconds(1))
                .disconnect_timeout(Seconds(1))
                .finish(|_req| {
                    info!("{:?}", _req);
                    let mut res = Response::Ok();
                    res.header("x-head", HeaderValue::from_static("dummy value!"));
                    future::ok::<_, io::Error>(res.body("Hello world!"))
                })
                .tcp()
        })?
        .run()
        .await
}
