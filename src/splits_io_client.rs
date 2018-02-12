use hyper::client::{Client as HyperClient, FutureResponse};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;
use hyper_tls::HttpsConnector;

type Connector = HttpsConnector<HttpConnector>;

pub struct Client {
    client: HyperClient<Connector>,
}

impl Client {
    pub fn new(handle: &Handle) -> Client {
        Client {
            client: HyperClient::configure()
                .connector(HttpsConnector::new(1, handle).expect(""))
                .build(handle),
        }
    }

    pub fn get(&self, uri: &str) -> FutureResponse {
        self.client.get(uri.parse().unwrap())
    }
}
