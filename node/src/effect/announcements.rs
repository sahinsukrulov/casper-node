//! Announcement effects.
//!
//! Announcements indicate new incoming data or events from various sources. See the top-level
//! module documentation for details.

use std::fmt::{self, Display, Formatter};

use crate::{
    types::{Deploy, ProtoBlock},
    utils::Source,
};

/// A networking layer announcement.
#[derive(Debug)]
#[must_use]
pub enum NetworkAnnouncement<I, P> {
    /// A payload message has been received from a peer.
    MessageReceived {
        /// The sender of the message
        sender: I,
        /// The message payload
        payload: P,
    },
}

impl<I, P> Display for NetworkAnnouncement<I, P>
where
    I: Display,
    P: Display,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NetworkAnnouncement::MessageReceived { sender, payload } => {
                write!(formatter, "received from {}: {}", sender, payload)
            }
        }
    }
}

/// An HTTP API server announcement.
#[derive(Debug)]
#[must_use]
pub enum ApiServerAnnouncement {
    /// A new deploy received.
    DeployReceived {
        /// The received deploy.
        deploy: Box<Deploy>,
    },
}

impl Display for ApiServerAnnouncement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ApiServerAnnouncement::DeployReceived { deploy } => {
                write!(formatter, "api server received {}", deploy.id())
            }
        }
    }
}

/// A `DeployAcceptor` announcement.
#[derive(Debug)]
pub enum DeployAcceptorAnnouncement<I> {
    /// A deploy which wasn't previously stored on this node has been accepted and stored.
    AcceptedNewDeploy {
        /// The new deploy.
        deploy: Box<Deploy>,
        /// The source (peer or client) of the deploy.
        source: Source<I>,
    },

    /// An invalid deploy was received.
    InvalidDeploy {
        /// The invalid deploy.
        deploy: Box<Deploy>,
        /// The source (peer or client) of the deploy.
        source: Source<I>,
    },
}

impl<I: Display> Display for DeployAcceptorAnnouncement<I> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeployAcceptorAnnouncement::AcceptedNewDeploy { deploy, source } => write!(
                formatter,
                "accepted new deploy {} from {}",
                deploy.id(),
                source
            ),
            DeployAcceptorAnnouncement::InvalidDeploy { deploy, source } => {
                write!(formatter, "invalid deploy {} from {}", deploy.id(), source)
            }
        }
    }
}

/// A consensus announcement.
#[derive(Debug)]
pub enum ConsensusAnnouncement {
    /// A block was proposed and will either be finalized or orphaned soon.
    Proposed(ProtoBlock),
    /// A block was finalized.
    // TODO: Replace with `FinalizedBlock`.
    Finalized(ProtoBlock),
    /// A block was orphaned.
    Orphaned(ProtoBlock),
}

impl Display for ConsensusAnnouncement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConsensusAnnouncement::Proposed(block) => {
                write!(formatter, "proposed proto block {}", block)
            }
            ConsensusAnnouncement::Finalized(block) => {
                write!(formatter, "finalized proto block {}", block)
            }
            ConsensusAnnouncement::Orphaned(block) => {
                write!(formatter, "orphaned proto block {}", block)
            }
        }
    }
}
