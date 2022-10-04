//! LoFiRe node data types
//!
//! Corresponds to the BARE schema

use lofire::types::*;
use lofire_net::types::*;
use lofire_repo::types::*;
use serde::{Deserialize, Serialize};

//
// OVERLAY MESSAGES
//

/// Overlay connection request
///
/// Sent to an existing overlay member to initiate a session
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OverlayConnect {
    V0(),
}

/// Overlay disconnection request
///
/// Sent to a connected overlay member to terminate a session
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OverlayDisconnect {
    V0(),
}

/// Content of TopicAdvertV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicAdvertContentV0 {
    /// Topic public key
    pub topic: PubKey,

    /// Peer public key
    pub peer: PeerId,
}

/// Topic advertisement by a publisher
///
/// Flooded to all peers in overlay
/// Creates subscription routing table entries
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicAdvertV0 {
    pub content: TopicAdvertContentV0,

    /// Signature over content by topic key
    pub sig: Signature,
}

/// Topic advertisement by a publisher
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicAdvert {
    V0(TopicAdvertV0),
}

/// Topic subscription request by a peer
///
/// Forwarded towards all publishers along subscription routing table entries
/// that are created by TopicAdverts
/// Creates event routing table entries along the path
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SubReqV0 {
    /// Random ID generated by the subscriber
    pub id: u64,

    /// Topic public key
    pub topic: PubKey,
}

/// Topic subscription request by a peer
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubReq {
    V0(SubReqV0),
}

// SubAck is defined in lofire-net/src/types.rs

/// Topic unsubscription request by a subscriber
///
/// A broker unsubscribes from upstream brokers
/// when it has no more subscribers left
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct UnsubReqV0 {
    /// Topic public key
    pub topic: PubKey,
}

/// Topic unsubscription request by a subscriber
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UnsubReq {
    V0(UnsubReqV0),
}

/// Topic unsubscription acknowledgement
/// Sent to the requestor in response to an UnsubReq
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct UnsubAckV0 {
    /// Topic public key
    pub topic: PubKey,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UnsubAck {
    V0(UnsubAckV0),
}

// Event, EventV0, EventContentV0, EventBodyV0, ChangeV0 are defined in lofire-net/src/types.rs

/// Object search in a pub/sub topic
///
/// Sent along the reverse path of a pub/sub topic
/// from a subscriber to all publishers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectSearchTopicV0 {
    /// Topic to forward the request in
    pub topic: PubKey,

    /// List of Object IDs to request
    pub ids: Vec<ObjectId>,

    /// Whether or not to include all children recursively in the response
    pub recursive: bool,

    /// List of Peer IDs the request traversed so far
    pub path: Vec<PeerId>,
}

/// Object request by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectSearchTopic {
    V0(ObjectSearchTopicV0),
}

/// Object search along a random walk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectSearchRandomV0 {
    /// List of Object IDs to request
    pub ids: Vec<ObjectId>,

    /// Whether or not to include all children recursively in the response
    pub recursive: bool,

    /// Number of random nodes to forward the request to at each step
    pub fanout: u8,

    /// List of Peer IDs the request traversed so far
    pub path: Vec<PeerId>,
}

/// Object request by ID using a random walk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectSearchRandom {
    V0(ObjectSearchRandomV0),
}

/// Response to an Object request
///
/// Follows request path with possible shortcuts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectResultV0 {
    /// Response path
    pub path: Vec<PeerId>,

    /// Resulting Object(s)
    pub payload: Vec<Object>,
}

/// Response to an Object request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectResult {
    V0(ObjectResultV0),
}

// BranchHeadsReqV0, BranchHeadsReq, BranchSyncReqV0, BranchSyncReq are defined in lofire-net/src/types.rs

/// Events the requestor needs, see EventReqV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NeedEventsV0 {
    /// Publisher ID
    pub publisher: Digest,

    /// First sequence number to request
    pub from: u32,

    /// Last sequence number to request
    pub to: u32,
}

// HaveEventsV0 is defined in lofire-net/src/types.rs

/// Request missed events for a pub/sub topic
/// for the specified range of publisher sequence numbers
///
/// In response an EventResp then a stream of Events are sent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventReqV0 {
    /// Topic public key
    pub topic: PubKey,

    /// Events needed by the requestor
    pub need: Vec<NeedEventsV0>,
}

/// Request missed events for a pub/sub topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventReq {
    V0(EventReqV0),
}

// EventRespV0 and EventResp are defined in lofire-net/src/types.rs

/// Content of OverlayRequestV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayRequestContentV0 {
    EventReq(EventReq),
    BranchHeadsReq(BranchHeadsReq),
    BranchSyncReq(BranchSyncReq),
}

/// Request sent to an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayRequestV0 {
    /// Request ID
    pub id: u64,

    /// Request content
    pub content: OverlayRequestContentV0,
}

/// Request sent to an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayRequest {
    V0(OverlayRequestV0),
}

/// Content of OverlayResponseV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayResponseContentV0 {
    Object(Object),
    EventResp(EventResp),
    Event(Event),
}

/// Request sent to an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result
    pub result: u8,

    /// Response content
    pub content: Option<OverlayResponseContentV0>,
}

/// Request sent to an OverlayRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayResponse {
    V0(OverlayResponseV0),
}

// PeerAdvert and OverlayId are defined in lofire-net/src/types.rs

/// Overlay session ID
///
/// Used as a component for key derivation.
/// Each peer generates it randomly when (re)joining the overlay network.
type SessionId = u64;

/// Content of OverlayMessagePaddedV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayMessageContentV0 {
    OverlayConnect(OverlayConnect),
    OverlayDisconnect(OverlayDisconnect),
    PeerAdvert(PeerAdvert),
    TopicAdvert(TopicAdvert),
    SubReq(SubReq),
    SubAck(SubAck),
    UnsubReq(UnsubReq),
    UnsubAck(UnsubAck),
    Event(Event),
    ObjectSearchTopic(ObjectSearchTopic),
    ObjectSearchRandom(ObjectSearchRandom),
    ObjectResult(ObjectResult),
    OverlayRequest(OverlayRequest),
    OverlayResponse(OverlayResponse),
}

/// Padded content of OverlayMessageV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayMessageContentPaddedV0 {
    pub content: OverlayMessageContentV0,

    /// Optional padding
    #[serde(with = "serde_bytes")]
    pub padding: Vec<u8>,
}

/// Overlay message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayMessageV0 {
    /// Overlay ID
    pub overlay: OverlayId,

    /// Session ID
    pub session: SessionId,

    /// Padded content encrypted with ChaCha20
    /// - overlay_secret: BLAKE3 derive_key ("LoFiRe Overlay BLAKE3 key",
    ///                                      repo_pubkey + repo_secret)
    /// - key: BLAKE3 derive_key ("LoFiRe OverlayMessage ChaCha20 key",
    ///                           overlay_secret + session_id)
    /// - nonce: per-session message sequence number of sending peer
    pub content: OverlayMessageContentPaddedV0,

    /// BLAKE3 MAC
    /// BLAKE3 keyed hash over the encrypted content
    /// - key:  BLAKE3 derive_key ("LoFiRe OverlayMessage BLAKE3 key",
    ///                            overlay_secret + session_id)
    pub mac: Digest,
}

/// Overlay message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayMessage {
    V0(OverlayMessageV0),
}

//
// BROKER STORAGE
//

/// A topic this node subscribed to in an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicV0 {
    /// Topic public key ID
    pub id: PubKey,

    /// Topic private key for publishers
    pub priv_key: Option<PrivKey>,

    /// Set of branch heads
    pub heads: Vec<ObjectId>,

    /// Number of local users that subscribed to the topic
    pub users: u32,
}

/// A topic this node subscribed to in an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Topic {
    V0(TopicV0),
}

/// An overlay this node joined
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayV0 {
    /// Overlay ID
    pub id: OverlayId,

    /// Overlay secret
    pub secret: SymKey,

    /// Known peers with connected flag
    pub peers: Vec<PeerAdvert>,

    /// Topics this node subscribed to in the overlay
    pub topics: Vec<Topic>,

    /// Number of local users that joined the overlay
    pub users: u32,

    /// Last access by any user
    pub last_access: Timestamp,
}

/// An overlay this node joined
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Overlay {
    V0(OverlayV0),
}

/// User account
///
/// Stored as user_pubkey -> Account
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountV0 {
    /// Authorized device pub keys
    pub authorized_keys: Vec<PubKey>,

    /// Admins can add/remove user accounts
    pub admin: bool,

    /// Overlays joined
    pub overlays: Vec<Overlay>,

    /// Topics joined, with publisher flag
    pub topics: Vec<Topic>,
}

/// User account
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Account {
    V0(AccountV0),
}
