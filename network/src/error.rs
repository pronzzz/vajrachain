use libp2p::gossipsub::PublishError;
use libp2p::gossipsub::SubscriptionError;
use libp2p::TransportError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Transport error: {0}")]
    Transport(#[from] TransportError<std::io::Error>),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Gossipsub subscription error: {0}")]
    Subscription(#[from] SubscriptionError),

    #[error("Gossipsub publish error: {0}")]
    Publish(#[from] PublishError),

    #[error("Swarm build error: {0}")]
    SwarmBuild(String),

    #[error("Behavior initialization error: {0}")]
    BehaviorInit(String),
}
