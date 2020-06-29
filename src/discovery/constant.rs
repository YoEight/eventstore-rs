use crate::internal::messaging::Msg;
use crate::types::Endpoint;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

pub(crate) async fn discover(
    mut consumer: mpsc::Receiver<Option<Endpoint>>,
    mut sender: mpsc::Sender<Msg>,
    endpoint: Endpoint,
) {
    while consumer.next().await.is_some() {
        let _ = sender.send(Msg::Establish(endpoint)).await;
    }
}
