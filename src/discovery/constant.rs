use crate::internal::messaging::Msg;
use crate::types::Endpoint;
use futures::future;
use futures::prelude::{Future, Sink, Stream};
use futures::sync::mpsc;
use tokio::spawn;

pub(crate) fn discover(
    consumer: mpsc::Receiver<Option<Endpoint>>,
    sender: mpsc::Sender<Msg>,
    endpoint: Endpoint,
) -> impl Future<Item = (), Error = ()> {
    struct State {
        sender: mpsc::Sender<Msg>,
        endpoint: Endpoint,
    }

    let initial = State { sender, endpoint };

    consumer
        .fold(initial, |state, _| {
            let send_endpoint = state
                .sender
                .clone()
                .send(Msg::Establish(state.endpoint))
                .then(|_| Ok(()));

            spawn(send_endpoint);

            future::ok(state)
        })
        .map(|_| ())
}
