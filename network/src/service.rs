use crate::behavior::VajraBehaviour;
use crate::messages::NetworkMessage;
use libp2p::{
    futures::StreamExt, gossipsub, mdns, noise, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId,
    Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info};

pub struct NetworkService {
    swarm: Swarm<VajraBehaviour>,
    command_receiver: mpsc::Receiver<NetworkCommand>,
    event_sender: mpsc::Sender<NetworkEvent>,
}

#[derive(Debug)]
pub enum NetworkCommand {
    BroadcastBlock(ledger::block::Block),
    BroadcastTransaction(ledger::transaction::Transaction),
    Dial(PeerId, Multiaddr),
}

#[derive(Debug)]
pub enum NetworkEvent {
    BlockReceived(ledger::block::Block),
    TransactionReceived(ledger::transaction::Transaction),
    PeerConnected(PeerId),
    NewListenAddr(Multiaddr),
}

use crate::error::NetworkError;

impl NetworkService {
    pub async fn new(
        local_key: libp2p::identity::Keypair,
    ) -> Result<
        (
            Self,
            mpsc::Sender<NetworkCommand>,
            mpsc::Receiver<NetworkEvent>,
        ),
        NetworkError,
    > {
        let local_peer_id = PeerId::from(local_key.public());
        info!("Local peer id: {:?}", local_peer_id);

        // Gossipsub configuration
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .map_err(|msg| NetworkError::BehaviorInit(format!("{:?}", msg)))?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|msg| NetworkError::BehaviorInit(format!("{:?}", msg)))?;

        // mDNS configuration
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
            .map_err(|e| NetworkError::BehaviorInit(e.to_string()))?;

        let behaviour = VajraBehaviour { gossipsub, mdns };

        let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| NetworkError::BehaviorInit(e.to_string()))?
            .with_behaviour(|_| behaviour)
            .map_err(|e| NetworkError::SwarmBuild(e.to_string()))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        let (command_sender, command_receiver) = mpsc::channel(32);
        let (event_sender, event_receiver) = mpsc::channel(32);

        Ok((
            Self {
                swarm,
                command_receiver,
                event_sender,
            },
            command_sender,
            event_receiver,
        ))
    }

    pub async fn run(mut self) {
        // Subscribe to topics
        let block_topic = gossipsub::IdentTopic::new("blocks");
        let tx_topic = gossipsub::IdentTopic::new("transactions");

        // Handle results safely (in prod, we'd retry or log better)
        let _ = self.swarm.behaviour_mut().gossipsub.subscribe(&block_topic);
        let _ = self.swarm.behaviour_mut().gossipsub.subscribe(&tx_topic);

        // Listen on all interfaces
        let _ = self
            .swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap()); // Changed to 127.0.0.1 for better local test reliability

        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(crate::behavior::VajraBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            info!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                            let _ = self.event_sender.send(NetworkEvent::PeerConnected(peer_id)).await;
                        }
                    },
                    SwarmEvent::Behaviour(crate::behavior::VajraBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source, message_id: _, message })) => {
                         if let Ok(msg) = bincode::deserialize::<NetworkMessage>(&message.data) {
                             info!("Received message from {propagation_source}: {:?}", msg);
                             match msg {
                                 NetworkMessage::Block(b) => {
                                     let _ = self.event_sender.send(NetworkEvent::BlockReceived(b)).await;
                                 }
                                 NetworkMessage::Transaction(t) => {
                                     let _ = self.event_sender.send(NetworkEvent::TransactionReceived(t)).await;
                                 }
                             }
                         }
                    },
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("Listening on {address}");
                        let _ = self.event_sender.send(NetworkEvent::NewListenAddr(address)).await;
                    },
                    SwarmEvent::OutgoingConnectionError { peer_id: Some(_peer_id), error: _error, .. } => {
                        // error!("Failed to dial {}: {}", _peer_id, _error);
                    },
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                         info!("Connected to {peer_id}");
                         self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                         let _ = self.event_sender.send(NetworkEvent::PeerConnected(peer_id)).await;
                    },
                    _ => {}
                },
                command = self.command_receiver.recv() => match command {
                    Some(NetworkCommand::BroadcastBlock(block)) => {
                        if let Ok(data) = bincode::serialize(&NetworkMessage::Block(block)) {
                            let topic = gossipsub::IdentTopic::new("blocks");
                            if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic, data) {
                                error!("Publish error: {e:?}");
                            }
                        }
                    },
                    Some(NetworkCommand::BroadcastTransaction(tx)) => {
                        if let Ok(data) = bincode::serialize(&NetworkMessage::Transaction(tx)) {
                            let topic = gossipsub::IdentTopic::new("transactions");
                            if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic, data) {
                                error!("Publish error: {e:?}");
                            }
                        }
                    },
                    Some(NetworkCommand::Dial(_peer_id, addr)) => {
                        if let Err(e) = self.swarm.dial(addr.clone()) {
                             error!("Dial error: {e:?}");
                        }
                    },
                    None => break,
                }
            }
        }
    }
}
