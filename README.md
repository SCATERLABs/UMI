# Project UMI(U and Me) – Peer-to-Peer Messaging with Rust & libp2p 

---

##  How Peer-to-Peer (P2P) Works
Most of the internet we use today is **client-server** based:  
- A client (your phone/browser) connects to a **central server** (like WhatsApp, Gmail, or Google).  
- That server processes and delivers messages/data.  

In contrast, a **Peer-to-Peer (P2P)** network has **no central server**.  
- Every device (peer) can act as both a client and a server.  
- Peers can discover each other and connect directly.  
- Communication can be **faster, private, and resilient** since there’s no single point of failure.  

 Real-world examples:  
- **BitTorrent** – peers share files directly.  
- **Blockchain** – decentralized transactions without banks.  
- **Skype (early days)** – direct calling between peers.  

This project is a **mini P2P messaging system** because sometimes I just want to **chat secretly with my favorite human** without some giant server spying on us.

---

## 🏗 Structure of the Project
Here’s the high-level structure of **Project UMI**:

```text
Project_UMI/
│── Cargo.toml # Rust dependencies
│── Cargo.lock
│── src/
│ └── main.rs # Core P2P logic (swarm, mdns, messaging)
│── target/ # Build output (ignored in git)
│── About.md # You are here!
```

## How its working:

- **Identity**: Each peer has a unique **Peer ID**.  
- **Discovery**: Peers find each other using **mDNS** (local Wi-Fi).  
- **Connection**: Once discovered, they open secure encrypted channels.  
- **Messaging**: You can send a message → peers receive and reply. 

```yaml
[ Peer A ] <---> [ Peer B ] <---> [ Peer C ]
                  ↕ ↕ ↕
       Local Wi-Fi Discovery (mDNS)

```

##  About This Project
**Project UMI** is my personal experiment to learn and build a **peer-to-peer chat system**.  
- Built in **Rust** 🦀 using [libp2p](https://libp2p.io/).  
- Works on **local Wi-Fi (LAN)** for now.  
- Each peer can send and receive messages directly.  
- Messages are exchanged using **Request/Response protocol**.  

This is **version 1** → basic functionality.  
Future versions will allow global peer-to-peer communication.

##  How to Run

#### 1. Install Rust
Get Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).  

#### 2️. Clone the Repository
```bash
git clone https://github.com/SCATERLABs/UMI.git
cd Project_UMI
```
#### 3. Run the Program:
```bash
cargo run
```
**You will see something likethis**:
I test with multiple peers(4) with in same wifi:
**PeerA**:
```yaml
Your Peer ID: 12D3KooWGHktar8HC2q1GhPazDNSZrehimNYS9Xgy23D4ch47Aag
Type a message and press Enter to send. Ctrl-C to quit.
Listening on /ip4/127.0.0.1/tcp/40357
Listening on /ip4/172.17.116.245/tcp/40357
Discovered peer: 12D3KooWQmFxzswRsKwES8xwuB7CVm2jM49jJijAsBSTunKhPFt3
Discovered peer: 12D3KooWE9LufgKDyy6HLRMLyUH4ms3eAqC8MxvyghN2Nw1cXZ3o
Discovered peer: 12D3KooWGGrzTYSixgRXvYQ93DuKAdxUkT9EjKSrKJQM5Pjrs66d
Received request: 'hii'
```
**PeerB**:
```yaml
Your Peer ID: 12D3KooWE9LufgKDyy6HLRMLyUH4ms3eAqC8MxvyghN2Nw1cXZ3o
Type a message and press Enter to send. Ctrl-C to quit.
Listening on /ip4/127.0.0.1/tcp/43331
Listening on /ip4/172.17.116.245/tcp/43331
Discovered peer: 12D3KooWGqhswM6pRAJkRWaSGposkNXw4Lcye9bVtgEdV4gwtBCE
Discovered peer: 12D3KooWQmFxzswRsKwES8xwuB7CVm2jM49jJijAsBSTunKhPFt3
Discovered peer: 12D3KooWGHktar8HC2q1GhPazDNSZrehimNYS9Xgy23D4ch47Aag
Discovered peer: 12D3KooWGGrzTYSixgRXvYQ93DuKAdxUkT9EjKSrKJQM5Pjrs66d
Peer expired: 12D3KooWGqhswM6pRAJkRWaSGposkNXw4Lcye9bVtgEdV4gwtBCE
Received request: 'hii'
```

**PeerC**:
```yaml
Listening on /ip4/127.0.0.1/tcp/46479
Discovered peer: 12D3KooWE9LufgKDyy6HLRMLyUH4ms3eAqC8MxvyghN2Nw1cXZ3o
Discovered peer: 12D3KooWGqhswM6pRAJkRWaSGposkNXw4Lcye9bVtgEdV4gwtBCE
Listening on /ip4/172.17.116.245/tcp/46479
Discovered peer: 12D3KooWGHktar8HC2q1GhPazDNSZrehimNYS9Xgy23D4ch47Aag
Discovered peer: 12D3KooWGGrzTYSixgRXvYQ93DuKAdxUkT9EjKSrKJQM5Pjrs66d
Peer expired: 12D3KooWGqhswM6pRAJkRWaSGposkNXw4Lcye9bVtgEdV4gwtBCE
hii
Sending message to 12D3KooWGGrzTYSixgRXvYQ93DuKAdxUkT9EjKSrKJQM5Pjrs66d: 'hii'
Sending message to 12D3KooWE9LufgKDyy6HLRMLyUH4ms3eAqC8MxvyghN2Nw1cXZ3o: 'hii'
Sending message to 12D3KooWGHktar8HC2q1GhPazDNSZrehimNYS9Xgy23D4ch47Aag: 'hii'
Received response: 'alive'
Received response: 'alive'
Received response: 'alive'
```

**PeerD**:
```yaml
Your Peer ID: 12D3KooWGGrzTYSixgRXvYQ93DuKAdxUkT9EjKSrKJQM5Pjrs66d
Type a message and press Enter to send. Ctrl-C to quit.
Listening on /ip4/127.0.0.1/tcp/34647
Listening on /ip4/172.17.116.245/tcp/34647
Discovered peer: 12D3KooWQmFxzswRsKwES8xwuB7CVm2jM49jJijAsBSTunKhPFt3
Discovered peer: 12D3KooWGHktar8HC2q1GhPazDNSZrehimNYS9Xgy23D4ch47Aag
Discovered peer: 12D3KooWE9LufgKDyy6HLRMLyUH4ms3eAqC8MxvyghN2Nw1cXZ3o
Received request: 'hii'
```