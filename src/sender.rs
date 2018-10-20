extern crate hyper;

use cfg::*;
use self::hyper::{Body, Client, Method, Request};
use self::hyper::client::HttpConnector;
use self::hyper::header::*;
use self::hyper::rt::{self, Future, Stream};

lazy_static! {
    static ref CLIENT : Client<HttpConnector, Body> = Client::new();
    static ref URI: hyper::Uri = NATURE_SERVER_ADDRESS.parse().unwrap();
}


pub fn send(json: String) {
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = URI.clone();
    req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let post = CLIENT.request(req).and_then(|res| {
        res.into_body().concat2()
    });
    let future = post
        .map(|_| ())
        .map_err(|err| error!("Error : {}", err));
    rt::run(future);
}

#[cfg(test)]
mod test {
    extern crate serde_json;

    use nature_common::util::setup_logger;
    use nature_db::*;
    use super::*;

        #[test]
    fn test_send_ok() {
        let _ = setup_logger();
        let delivery = RawDelivery::new(&"hello".to_string(), "/hello", 2).unwrap();
        let json = serde_json::to_string(&delivery).unwrap();
        send(json.clone());
        send(json)
    }

    #[test]
    fn test_send_400() {
        let _ = setup_logger();
        send("hello".to_string())
    }
}