use ledger::block::{Block, BlockHeader};
use libp2p::identity::Keypair;
use libp2p::{Multiaddr, PeerId};
use network::service::{NetworkCommand, NetworkEvent, NetworkService};
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn test_p2p_gossip() {
    // Setup Node 1
    let key1 = Keypair::generate_ed25519();
    let _peer_id1 = PeerId::from(key1.public());
    let (node1, sender1, mut events1) = NetworkService::new(key1).await.unwrap();
    tokio::spawn(node1.run());

    // Setup Node 2
    let key2 = Keypair::generate_ed25519();
    let peer_id2 = PeerId::from(key2.public());
    let (node2, _sender2, mut events2) = NetworkService::new(key2).await.unwrap();
    tokio::spawn(node2.run());

    println!("Nodes spawned. Getting listen addresses...");

    let mut addr1: Option<Multiaddr> = None;
    let mut addr2: Option<Multiaddr> = None;

    // Get Listen Addr for Node 1
    if let Ok(Some(NetworkEvent::NewListenAddr(addr))) =
        timeout(Duration::from_secs(2), events1.recv()).await
    {
        addr1 = Some(addr);
    }

    // Get Listen Addr for Node 2
    if let Ok(Some(NetworkEvent::NewListenAddr(addr))) =
        timeout(Duration::from_secs(2), events2.recv()).await
    {
        addr2 = Some(addr);
    }

    let addr1 = addr1.expect("Node 1 failed to get listen addr");
    let addr2 = addr2.expect("Node 2 failed to get listen addr");

    println!("Node 1 listening on {addr1}");
    println!("Node 2 listening on {addr2}");

    // Manual Dial: Node 1 dials Node 2
    println!("Node 1 dialing Node 2...");
    sender1
        .send(NetworkCommand::Dial(peer_id2, addr2))
        .await
        .unwrap();

    // Wait for connection
    let connect_timeout = Duration::from_secs(5);
    let wait_for_connect = async {
        loop {
            if let Some(event) = events1.recv().await {
                if let NetworkEvent::PeerConnected(pid) = event {
                    if pid == peer_id2 {
                        println!("Node 1 connected to Node 2");
                        break;
                    }
                }
            }
        }
    };

    if let Err(_) = timeout(connect_timeout, wait_for_connect).await {
        panic!("Timed out waiting for connection");
    }

    // Give Gossipsub a moment to exchange subscriptions
    sleep(Duration::from_secs(2)).await;

    // Node 1 broadcasts a block
    let dummy_block = Block {
        header: BlockHeader {
            parent_hash: "000".to_string(),
            timestamp: 123456,
            slot: 1,
            state_root: "root".to_string(),
            transactions_root: "tx_root".to_string(),
            validator_public_key: vec![],
            signature: vec![],
        },
        transactions: vec![],
    };

    println!("Node 1 broadcasting block...");
    sender1
        .send(NetworkCommand::BroadcastBlock(dummy_block.clone()))
        .await
        .unwrap();

    // Node 2 should receive it
    println!("Node 2 waiting for block...");
    let receive_timeout = Duration::from_secs(5);
    let wait_for_block = async {
        loop {
            if let Some(event) = events2.recv().await {
                if let NetworkEvent::BlockReceived(block) = event {
                    assert_eq!(block.header.timestamp, 123456);
                    println!("Node 2 received block!");
                    break;
                }
            }
        }
    };

    if let Err(_) = timeout(receive_timeout, wait_for_block).await {
        panic!("Timed out waiting for Node 2 to receive block");
    }
}
