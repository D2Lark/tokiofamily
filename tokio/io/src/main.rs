use tokio::io;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() -> io::Result<()>{
    let tcp = TcpListener::bind("127.0.0.1:6379").await?;
    loop{
        let (mut socket,_) = tcp.accept().await?;
        tokio::spawn(            async move{
            let (mut rd,mut wr) =  socket.split();
            if io::copy(&mut rd,&mut wr).await.is_err(){
                eprintln!("fail to copy");
            }
        });
    }
}