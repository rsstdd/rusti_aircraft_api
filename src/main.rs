extern crate gotham;

use gotham::state::State;

const HELLO_WORLD: &'static str = "Hello World";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

pub fn main() {
    let addr = "127.0.0.1:8000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
