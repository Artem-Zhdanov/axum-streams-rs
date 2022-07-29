use crate::stream_format::StreamingFormat;
use axum::body::HttpBody;
use axum::response::{IntoResponse, Response};
use futures_util::stream::BoxStream;
use http::HeaderMap;
use std::fmt::Formatter;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamBodyWithFormat<'a> {
    stream: BoxStream<'a, Result<axum::body::Bytes, axum::Error>>,
    trailers: Option<HeaderMap>,
}

impl<'a> std::fmt::Debug for StreamBodyWithFormat<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StreamBodyWithFormat")
    }
}

impl<'a> StreamBodyWithFormat<'a> {
    /// Create a new `StreamBodyWithFormat` providing a stream of your objects in the specified format.
    pub fn new<T, FMT>(stream_format: FMT, stream: BoxStream<'a, T>) -> Self
    where
        FMT: StreamingFormat<T>,
    {
        Self {
            stream: stream_format.bytes_stream(stream),
            trailers: stream_format.http_response_trailers(),
        }
    }
}

impl IntoResponse for StreamBodyWithFormat<'static> {
    fn into_response(self) -> Response {
        Response::new(axum::body::boxed(self))
    }
}

impl<'a> HttpBody for StreamBodyWithFormat<'a> {
    type Data = axum::body::Bytes;
    type Error = axum::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        use futures_util::Stream;
        Pin::new(&mut self.stream).poll_next(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(self.trailers.clone()))
    }
}
