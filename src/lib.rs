extern crate chan;
extern crate core;
extern crate bytes;
extern crate uuid;
extern crate chrono;
extern crate timer;
extern crate time;

mod internal;
pub mod client;

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use client::Client;

    #[test]
    fn it_works() {
        let client = Client::new("127.0.0.1:32770".parse().unwrap());

        client.start();

        loop {
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
