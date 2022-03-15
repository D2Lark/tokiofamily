
use  bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc,oneshot};
use tokio::time::{Duration,sleep};

#[derive(Debug)]
enum Command{
    Get{
       key:String,
       resp:Responder<Option<Bytes>>, 
    }, 
    Set{
        key:String,
        value:Bytes,
        resp:Responder<()>,
    },
}
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    
    let manager = tokio::spawn(async move{
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await{
            match cmd{
                Command::Get { key, resp } =>{
                    let res =client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, value, resp} =>{
                    let res =  client.set(&key,value).await;
                    let _ = resp.send(res);
                }
            }

        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx,resp_rx) = oneshot::channel();
        
        let cmd = Command::Get{
            key:"Hello".to_string(),
            resp:resp_tx,
        };

        if tx.send(cmd).await.is_err(){
            eprint!("get conn shut down");
            return
        }

        let res = resp_rx.await;
        println!("got result from get cmd :{:?}",res);
    });

    
    let t2 = tokio::spawn(async move {
        let (resp_tx,resp_rx) = oneshot::channel();
        
        let cmd = Command::Set{
            key:"Hello".to_string(),
            value:"World".into(),
            resp:resp_tx,
        };

        if tx2.send(cmd).await.is_err(){
            eprint!("set conn shut down");
            return
        }

        let res = resp_rx.await;
        println!("got result from set cmd :{:?}",res);
    });
    t1.await.unwrap();

    sleep(Duration::from_secs(3)).await;
    t2.await.unwrap();

    manager.await.unwrap();
}
