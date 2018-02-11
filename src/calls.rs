use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use Parse;

pub fn get_response<V, F>(uri: &str, f: F)
where
    F: FnOnce(V),
    V: Parse<V>,
{
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(1, &core.handle()).expect(""))
        .build(&core.handle());
    let uri = uri.parse().unwrap();
    let work = client.get(uri).map(|res| {
        println!("{:?}", res.status());
        let _ = res.body().concat2().map(|chunk| {
            let chunk = chunk.to_vec();
            let s = String::from_utf8(chunk).expect("err");
            let v = V::parse(&s).unwrap();
            f(v);
        });
    });
    core.run(work).unwrap();
}

#[test]
fn test_get() {
    use super::User;
    get_response("https://splits.io/api/v3/users/555", |u: User| {
        println!("{:#?}", u)
    });
    assert!(false);
}
