use futures_core::Stream;
use std::io::Read;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct FutureStream {
    reader: Arc<Mutex<dyn Read + Send>>,
}

impl FutureStream {
    pub fn new(reader: Arc<Mutex<dyn Read + Send>>) -> Self {
        FutureStream { reader }
    }
}

impl Stream for FutureStream {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0; 40960];

        match self.reader.lock().unwrap().read(&mut buffer) {
            Ok(0) => Poll::Ready(None),
            Ok(n) => {
                buffer.truncate(n);
                Poll::Ready(Some(Ok(buffer)))
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}
