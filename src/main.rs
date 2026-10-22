use std::time::Duration;

use http_body_util::StreamBody;
use hyper::body::{Bytes, Frame, Incoming};
use hyper::{Request, Response, service};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::watch::Receiver;
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::{Stream, StreamExt};

use crate::generators::Generator;
use crate::generators::jel::JelGenerator;
use crate::generators::test_square::TestSquare;
use crate::gif::LogicalScreenDescriptorPackedField;

mod frame;
mod generators;
mod gif;
mod lzw;

fn generate_gif(mut running_rx: Receiver<bool>) -> mpsc::UnboundedReceiver<Vec<u8>> {
    let (tx, rx) = mpsc::unbounded_channel();

    tx.send(
        [
            gif::header().as_ref(),
            gif::logical_screen_descriptor(
                88,
                31,
                LogicalScreenDescriptorPackedField {
                    global_color_table_flag: false,
                    color_resolution: 7,
                    sort_flag: false,
                    size_of_global_color_table: 0,
                },
                0,
                0,
            )
            .as_ref(),
        ]
        .concat(),
    )
    .unwrap();

    tokio::spawn(async move {
        'outer: loop {
            for mut generator in [
                Box::new(JelGenerator::new()) as Box<dyn Generator>,
                Box::new(TestSquare::new(50)),
            ] {
                while *running_rx.borrow() {
                    let Some(frame) = generator.next_frame() else {
                        break;
                    };

                    let Ok(()) = tx.send(frame.to_bytes()) else {
                        return;
                    };

                    tokio::select! {
                        biased;
                        () = tokio::time::sleep(Duration::from_millis(20)) => (),
                        _ = running_rx.changed() => break 'outer,
                    };
                }
            }
        }

        tx.send([gif::trailer()].into()).ok();
    });

    rx
}

fn stream_gif(
    _: Request<Incoming>,
    running_rx: Receiver<bool>,
) -> Response<StreamBody<impl Stream<Item = Result<Frame<Bytes>, !>>>> {
    let stream = UnboundedReceiverStream::new(generate_gif(running_rx))
        .map(|bytes| Ok(Frame::data(bytes.into())));

    Response::builder()
        .header("Content-Type", "image/gif")
        .header("Cache-Control", "no-store")
        .body(StreamBody::new(stream))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let (running_tx, mut running_rx) = watch::channel(true);

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        running_tx.send(false).unwrap();
        eprintln!("stop!!!");
    });

    let listener = TcpListener::bind(("localhost", 8080)).await.unwrap();

    while *running_rx.borrow() {
        let (stream, _) = tokio::select! {
            biased;
            result = listener.accept() => result.unwrap(),
            _ = running_rx.changed() => break,
        };

        let running_rx = running_rx.clone();

        tokio::task::spawn(async move {
            Builder::new(TokioExecutor::new())
                .serve_connection(
                    TokioIo::new(stream),
                    service::service_fn(|request| {
                        let running_rx = running_rx.clone();
                        async move { Ok::<_, !>(stream_gif(request, running_rx)) }
                    }),
                )
                .await
                .ok()
        });
    }
}
