use futures_core::Stream;
use std::fs::File;
use std::io::Read;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct BrotliDecompressStream {
    decompressor: brotli::Decompressor<File>,
}

impl BrotliDecompressStream {
    pub fn new(decompressor: brotli::Decompressor<File>) -> Self {
        BrotliDecompressStream { decompressor }
    }
}

impl Stream for BrotliDecompressStream {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0; 40960];

        match self.decompressor.read(&mut buffer) {
            Ok(0) => Poll::Ready(None),
            Ok(n) => {
                buffer.truncate(n);
                Poll::Ready(Some(Ok(buffer)))
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}
