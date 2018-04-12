use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Sender, channel };
use tokio_core::reactor::Core;

use internal::driver::{ Driver, Report };
use internal::messaging::Msg;
use internal::metadata::StreamMetadata;
use command;
use discovery::StaticDiscovery;
use types::Settings;

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
}

impl Client {
    pub fn new(settings: Settings, addr: SocketAddrV4) -> Client {
        let (sender, recv) = channel(500);
        let disc           = Box::new(StaticDiscovery::new(addr));

        let tx     = sender.clone();
        let handle = spawn(move || {
            let mut core   = Core::new().unwrap();
            let     handle = core.handle();

            let mut driver = Driver::new(settings, disc, sender, handle.clone());
            let mut ticker = None;

            let worker = recv.for_each(move |msg| {
                match msg {
                    Msg::Start => {
                        ticker = Some(driver.start());
                    },

                    Msg::Shutdown => {
                        println!("Shutting down...");
                        return Err(());
                    },

                    Msg::Establish(endpoint) =>
                        driver.on_establish(endpoint),

                    Msg::Established(id) => {
                        driver.on_established(id);
                    },

                    Msg::ConnectionClosed(conn_id, error) => {
                        driver.on_connection_closed(conn_id, error);
                    },

                    Msg::Arrived(pkg) => {
                        driver.on_package_arrived(pkg);
                    },

                    Msg::Tick => {
                        if let Report::Quit = driver.on_tick() {
                            driver.close_connection();
                            return Err(())
                        }
                    },

                    Msg::NewOp(op) => {
                        driver.on_new_op(op);
                    }
                };

                Ok(())
            });

            // TODO - Handle more gracefully when the driver quits.
            core.run(worker).unwrap();
        });

        Client {
            worker: handle,
            sender: tx,
        }
    }

    pub fn start(&self) {
        self.sender.clone().send(Msg::Start).wait().unwrap();
    }

    pub fn write_events(&self, stream: String) -> command::WriteEvents {
        command::WriteEvents::new(self.sender.clone(), stream)
    }

    pub fn write_stream_metadata(&self, stream: String, metadata: StreamMetadata) -> command::WriteStreamData {
        command::WriteStreamData::new(self.sender.clone(), stream, metadata)
    }

    pub fn read_event(&self, stream: String, event_number: i64) -> command::ReadEvent {
        command::ReadEvent::new(self.sender.clone(), stream, event_number)
    }

    pub fn read_stream_metadata(&self, stream: String) -> command::ReadStreamData {
        command::ReadStreamData::new(self.sender.clone(), stream)
    }

    pub fn start_transaction(&self, stream: String) -> command::TransactionStart {
        command::TransactionStart::new(self.sender.clone(), stream)
    }

    pub fn read_stream(&self, stream: String) -> command::ReadStreamEvents {
        command::ReadStreamEvents::new(self.sender.clone(), stream)
    }

    pub fn read_all(&self) -> command::ReadAllEvents {
        command::ReadAllEvents::new(self.sender.clone())
    }

    pub fn delete_stream(&self, stream: String) -> command::DeleteStream {
        command::DeleteStream::new(self.sender.clone(), stream)
    }

    pub fn shutdown(&self) {
        self.sender.clone().send(Msg::Shutdown).wait().unwrap();
    }

    pub fn wait_till_closed(self) {
        self.worker.join().unwrap();
    }
}
