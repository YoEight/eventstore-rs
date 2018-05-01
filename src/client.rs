use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Sender, channel };
use protobuf::Chars;
use tokio_core::reactor::Core;

use internal::driver::{ Driver, Report };
use internal::messaging::Msg;
use internal::commands;
use discovery::StaticDiscovery;
use types::{ StreamMetadata, Settings };

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
    settings: Settings,
}

impl Client {
    pub fn new(settings: Settings, addr: SocketAddrV4) -> Client {
        let (sender, recv) = channel(500);
        let disc           = Box::new(StaticDiscovery::new(addr));

        let tx     = sender.clone();
        let setts  = settings.clone();
        let handle = spawn(move || {
            let mut core   = Core::new().unwrap();
            let     handle = core.handle();

            let mut driver = Driver::new(&setts, disc, sender, handle.clone());
            let mut ticker = None;

            let worker = recv.for_each(move |msg| {
                match msg {
                    Msg::Start => {
                        ticker = Some(driver.start());
                    },

                    Msg::Shutdown => {
                        debug!("Client is shutting down...");
                        // TODO - Implement graceful exit but handling all the
                        // reponses we got so far.
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
                    },

                    Msg::Send(pkg) => {
                        driver.on_send_pkg(pkg);
                    },
                };

                Ok(())
            });

            // TODO - Handle more gracefully when the driver quits.
            let _ = core.run(worker);

            info!("Client is closed");
        });

        Client {
            worker: handle,
            sender: tx,
            settings,
        }
    }

    pub fn start(&self) {
        self.sender.clone().send(Msg::Start).wait().unwrap();
    }

    pub fn write_events<S>(&self, stream: S) -> commands::WriteEvents
        where S: Into<Chars>
    {
        commands::WriteEvents::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn write_stream_metadata<S>(&self, stream: S, metadata: StreamMetadata)
        -> commands::WriteStreamMetadata
        where S: Into<Chars>
    {
        commands::WriteStreamMetadata::new(self.sender.clone(), stream, metadata, &self.settings)
    }

    pub fn read_event<S>(&self, stream: S, event_number: i64) -> commands::ReadEvent
        where S: Into<Chars>
    {
        commands::ReadEvent::new(self.sender.clone(), stream, event_number, &self.settings)
    }

    pub fn read_stream_metadata<S>(&self, stream: S) -> commands::ReadStreamMetadata
        where S: Into<Chars>
    {
        commands::ReadStreamMetadata::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn start_transaction<S>(&self, stream: S) -> commands::TransactionStart
        where S: Into<Chars>
    {
        commands::TransactionStart::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn read_stream<S>(&self, stream: S) -> commands::ReadStreamEvents
        where S: Into<Chars>
    {
        commands::ReadStreamEvents::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn read_all(&self) -> commands::ReadAllEvents {
        commands::ReadAllEvents::new(self.sender.clone(), &self.settings)
    }

    pub fn delete_stream<S>(&self, stream: S) -> commands::DeleteStream
        where S: Into<Chars>
    {
        commands::DeleteStream::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn subcribe_to_stream<S>(&self, stream_id: S) -> commands::SubscribeToStream
        where S: Into<Chars>
    {
        commands::SubscribeToStream::new(self.sender.clone(), stream_id, &self.settings)
    }

    pub fn subscribe_to_stream_from<S>(&self, stream: S) -> commands::RegularCatchupSubscribe
        where S: Into<Chars>
    {
        commands::RegularCatchupSubscribe::new(self.sender.clone(), stream, &self.settings)
    }

    pub fn subscribe_to_all_from(&self) -> commands::AllCatchupSubscribe
    {
        commands::AllCatchupSubscribe::new(self.sender.clone(), &self.settings)
    }

    pub fn create_persistent_subscription<S>(&self, stream_id: S, group_name: S) -> commands::CreatePersistentSubscription
        where S: Into<Chars>
    {
        commands::CreatePersistentSubscription::new(stream_id, group_name, self.sender.clone(), &self.settings)
    }

    pub fn shutdown(&self) {
        self.sender.clone().send(Msg::Shutdown).wait().unwrap();
    }

    pub fn wait_till_closed(self) {
        self.worker.join().unwrap();
    }
}
