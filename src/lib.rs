use tokio::{net::{TcpStream}, io};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const LISTEN_ADDRESS: &str = "0.0.0.0:3000";
pub const TARGET_ADDRESS: &str = "127.0.0.1:6767";

pub async fn proxy(mut stream: TcpStream) -> Result<()> {
    let mut server_stream = TcpStream::connect(TARGET_ADDRESS).await?;

    let (mut client_read, mut client_write) = stream.split();
    let (mut server_read, mut server_write) = server_stream.split();

    tokio::try_join!(
        io::copy(&mut client_read, &mut server_write),
        io::copy(&mut server_read, &mut client_write)
    )?;

    Ok(())
}