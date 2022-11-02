use std::{io, net::SocketAddr};

use bytes::BytesMut;
use futures::future::Future;
use tokio_executor::{DefaultExecutor, Executor};
use tokio_tcp::TcpStream;

use crate::{
    protocol::{
        connection::{Connection, ConnectionHandle, OperationError},
        service::EmptyService,
    },
    request::Request,
    response::Response,
};

pub struct Client {
    handle: ConnectionHandle,
    server_address: SocketAddr,
}

impl Client {
    pub fn connect(server_address: SocketAddr) -> impl Future<Item = Client, Error = io::Error> {
        TcpStream::connect(&server_address).and_then(move |tcp_stream| {
            let mut executor = DefaultExecutor::current();
            let (connection, handler, handle) = Connection::new::<EmptyService>(tcp_stream, None);

            executor.spawn(Box::new(connection)).unwrap();

            if let Some(handler) = handler {
                executor.spawn(Box::new(handler)).unwrap();
            }
            Ok(Client {
                handle,
                server_address,
            })
        })
    }

    pub fn server_address(&self) -> &SocketAddr {
        &self.server_address
    }

    pub fn send_request<R, B>(
        &mut self,
        request: R,
    ) -> impl Future<Item = Response<BytesMut>, Error = OperationError>
    where
        R: Into<Request<B>>,
        B: AsRef<[u8]>,
    {
        self.handle.send_request(request)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bounds() {
        fn check_bounds<T: Send + Send>() {}

        check_bounds::<Client>();
    }
}
