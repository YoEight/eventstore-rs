extern crate chan;
extern crate core;
extern crate bytes;
extern crate uuid;
extern crate chrono;
extern crate timer;
extern crate time;
extern crate protobuf;

mod internal;
pub mod client;

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use client::Client;
    use internal::types::{ Credentials, Settings };

    #[test]
    fn it_works() {
        let mut settings = Settings::default();
        let login        = "admin".to_owned();
        let passw        = "changeit".to_owned();

        settings.default_user = Some(Credentials { login: login, password: passw });

        let client   = Client::new(settings, "127.0.0.1:1113".parse().unwrap());

        client.start();

        loop {
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
