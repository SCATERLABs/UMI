use async_trait::async_trait;
use futures::{io::{self, AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt}, stream::StreamExt};
use libp2p::{
    identity,
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig, Event as MdnsEvent},
    noise, tcp, yamux,
    // FIX: Correctly import NetworkBehaviour from the swarm module
    swarm::{NetworkBehaviour, SwarmEvent},
    request_response::{self, ProtocolSupport},
    // The unused PeerId warning is gone because this is needed now.
    PeerId, SwarmBuilder,
};
// New imports for reading from stdin and managing peers
use std::{error::Error, iter, collections::HashSet};
use tokio::{io::{stdin, AsyncBufReadExt, BufReader}, sync::mpsc};


// The Codec and Protocol definitions remain the same...
#[derive(Debug, Clone)]
pub struct MyProtocol();
#[derive(Clone, Default)]
pub struct MyCodec();
pub type MyRequest = String;
pub type MyResponse = String;
impl AsRef<str> for MyProtocol {
    fn as_ref(&self) -> &str {
        "/my-protocol/1.0"
    }
}
#[async_trait]
impl request_response::Codec for MyCodec {
    type Protocol = MyProtocol;
    type Request = MyRequest;
    type Response = MyResponse;
    async fn read_request<T>(&mut self, _: &MyProtocol, io: &mut T) -> io::Result<Self::Request> where T: AsyncRead + Unpin + Send {
        let mut vec = Vec::new(); io.read_to_end(&mut vec).await?; Ok(String::from_utf8(vec).unwrap())
    }
    async fn read_response<T>(&mut self, _: &MyProtocol, io: &mut T) -> io::Result<Self::Response> where T: AsyncRead + Unpin + Send {
        let mut vec = Vec::new(); io.read_to_end(&mut vec).await?; Ok(String::from_utf8(vec).unwrap())
    }
    async fn write_request<T>(&mut self, _: &MyProtocol, io: &mut T, req: Self::Request) -> io::Result<()> where T: AsyncWrite + Unpin + Send {
        io.write_all(req.as_bytes()).await?; Ok(())
    }
    async fn write_response<T>(&mut self, _: &MyProtocol, io: &mut T, res: Self::Response) -> io::Result<()> where T: AsyncWrite + Unpin + Send {
        io.write_all(res.as_bytes()).await?; Ok(())
    }
}


#[derive(NetworkBehaviour)]
struct MyBehaviour {
    mdns: Mdns,
    req_res: request_response::Behaviour<MyCodec>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();

    let mut swarm = SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_dns()?
        .with_behaviour(|key| {
            let peer_id = key.public().to_peer_id();
            let mdns = Mdns::new(MdnsConfig::default(), peer_id).expect("can create mdns");
            let req_res = request_response::Behaviour::new(
                iter::once((MyProtocol(), ProtocolSupport::Full)),
                request_response::Config::default(),
            );
            MyBehaviour { mdns, req_res }
        })?
        .build();

    let (tx, mut rx) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        let mut reader = BufReader::new(stdin()).lines();
        while let Some(line) = reader.next_line().await.unwrap() {
            if let Err(e) = tx.send(line).await {
                eprintln!("Failed to send input to swarm: {e}");
            }
        }
    });

    println!("Your Peer ID: {}", swarm.local_peer_id());
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("Type a message and press Enter to send. Ctrl-C to quit.");

    let mut discovered_peers = HashSet::new();

    loop {
        tokio::select! {
            Some(line) = rx.recv() => {
                if discovered_peers.is_empty() {
                    println!("No peers discovered yet. Message not sent.");
                    continue;
                }
                for peer_id in &discovered_peers {
                    println!("Sending message to {peer_id}: '{line}'");
                    swarm
                        .behaviour_mut()
                        .req_res
                        .send_request(peer_id, line.clone());
                }
            },
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {address}");
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                        for (peer_id, _addr) in peers {
                            println!("Discovered peer: {peer_id}");
                            discovered_peers.insert(peer_id);
                        }
                    }
                     SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Expired(peers))) => {
                        for (peer_id, _addr) in peers {
                            println!("Peer expired: {peer_id}");
                            discovered_peers.remove(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::ReqRes(request_response::Event::Message {
                        message: request_response::Message::Request { request, channel, .. }, ..
                    })) => {
                        println!("Received request: '{request}'");
                        if let Err(e) = swarm.behaviour_mut().req_res.send_response(channel, "alive".to_string()) {
                            eprintln!("Failed to send response: {e}");
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::ReqRes(request_response::Event::Message {
                        message: request_response::Message::Response { response, .. }, ..
                    })) => {
                        println!("Received response: '{response}'");
                    }
                    _ => {}
                }
            }
        }
    }
}