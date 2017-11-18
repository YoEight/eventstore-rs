use std::net::{ TcpStream, SocketAddrV4 };
use std::thread::{ JoinHandle, spawn };

use chan::{ Sender, Receiver, async };
use uuid::Uuid;

use internal::messaging::Msg;
use internal::package::Pkg;

pub struct Connection {
    pub id:     Uuid,
        sender: Sender<Pkg>,
        worker: JoinHandle<()>,
}

impl Connection {
    pub fn new(bus: Sender<Msg>, addr: SocketAddrV4) -> Connection {
        let (sender, recv) = async();
        let id             = Uuid::new_v4();
        let worker         = spawn(move || Connection::create_conn(id, recv, bus, addr));

        Connection {
            id:     id,
            sender: sender,
            worker: worker,
        }
    }

    fn create_conn(id: Uuid, rx: Receiver<Pkg>, bus: Sender<Msg>, addr: SocketAddrV4) {
        let stream = TcpStream::connect(addr).unwrap();

        bus.send(Msg::Established(id));
    }

    pub fn enqueue(&self, pkg: Pkg) {
        self.sender.send(pkg);
    }
}
