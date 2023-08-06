use error::XmlRpcError;

pub mod server;
pub mod client;
pub mod error;

pub type XmlRpcResult<T> = Result<T, XmlRpcError>;

#[macro_export]
macro_rules! header {
    () => {
        "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}"
    };
}