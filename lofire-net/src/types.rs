//! LoFiRe network protocol types
//!
//! Corresponds to the BARE schema

use lofire::types::*;
use serde::{Deserialize, Serialize};

//
// COMMON TYPES FOR MESSAGES
//

/// Peer ID: public key of the node
pub type PeerId = PubKey;

/// Overlay ID
///
/// - for public overlays that need to be discovered by public key:
///   BLAKE3 hash over the repository public key
/// - for private overlays:
///   BLAKE3 keyed hash over the repository public key
///   - key: BLAKE3 derive_key ("LoFiRe OverlayId BLAKE3 key", repo_secret)
pub type OverlayId = Digest;

/// Overlay session ID
///
/// Used as a component for key derivation.
/// Each peer generates it randomly when (re)joining the overlay network.
pub type SessionId = u64;

/// Topic ID: public key of the topic
pub type TopicId = PubKey;

/// User ID: user account for broker
pub type UserId = PubKey;

/// Client ID: client of a user
pub type ClientId = PubKey;

/// IPv4 address
pub type IPv4 = [u8; 4];

/// IPv6 address
pub type IPv6 = [u8; 16];

/// IP address
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum IP {
    IPv4(IPv4),
    IPv6(IPv6),
}

/// IP transport protocol
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum IPTransportProtocol {
    TLS,
    QUIC,
}

/// IP transport address
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct IPTransportAddr {
    pub ip: IP,
    pub port: u16,
    pub protocol: IPTransportProtocol,
}

/// Network address
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetAddr {
    IPTransport(IPTransportAddr),
}

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
    pub topic: TopicId,

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
    pub sig: Sig,
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
    pub topic: TopicId,
}

/// Topic subscription request by a peer
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubReq {
    V0(SubReqV0),
}

/// Topic subscription acknowledgement by a publisher
///
/// Sent to all subscribers in an Event.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SubAckV0 {
    /// SubReq ID to acknowledge
    pub id: u64,
}

/// Topic subscription acknowledgement by a publisher
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubAck {
    V0(SubAckV0),
}

/// Topic unsubscription request by a subscriber
///
/// A broker unsubscribes from upstream brokers
/// when it has no more subscribers left
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct UnsubReqV0 {
    /// Topic public key
    pub topic: TopicId,
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
    pub topic: TopicId,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UnsubAck {
    V0(UnsubAckV0),
}

/// Branch change notification
/// Contains a chunk of a newly added Commit or File referenced by a commit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeV0 {
    /// Block with encrypted content
    pub content: Block,

    /// Encrypted key for the Commit object in content
    /// Only set for the root block of the object
    /// The key is encrypted using ChaCha20:
    /// - key: BLAKE3 derive_key ("LoFiRe Event ObjectRef ChaCha20 key",
    ///                           branch_pubkey + branch_secret + publisher_pubkey)
    /// - nonce: commit_seq
    pub key: Option<SymKey>,
}

/// Body of EventContentV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EventBodyV0 {
    SubAck,
    Change,
}

/// Content of EventV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EventContentV0 {
    /// Pub/sub topic
    pub topic: TopicId,

    /// Publisher pubkey hash
    /// BLAKE3 keyed hash over branch member pubkey
    /// - key: BLAKE3 derive_key ("LoFiRe Event publisher BLAKE3 key",
    ///                           repo_pubkey + repo_secret +
    ///                           branch_pubkey + branch_secret)
    pub publisher: Digest,

    /// Commit sequence number of publisher
    pub seq: u32,

    /// Event body
    pub body: EventBodyV0,
}

/// Pub/sub event published in a topic
///
/// Forwarded along event routing table entries
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EventV0 {
    pub content: EventContentV0,

    /// Signature over content by topic key
    pub sig: Sig,
}

/// Pub/sub event published in a topic
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Event {
    V0(EventV0),
}

/// Object search in a pub/sub topic
///
/// Sent along the reverse path of a pub/sub topic
/// from a subscriber to all publishers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockSearchTopicV0 {
    /// Topic to forward the request in
    pub topic: TopicId,

    /// List of Object IDs to request
    pub ids: Vec<ObjectId>,

    /// Whether or not to include all children recursively in the response
    pub include_children: bool,

    /// List of Peer IDs the request traversed so far
    pub path: Vec<PeerId>,
}

/// Object request by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockSearchTopic {
    V0(BlockSearchTopicV0),
}

/// Block search along a random walk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockSearchRandomV0 {
    /// List of Block IDs to request
    pub ids: Vec<BlockId>,

    /// Whether or not to include all children recursively in the response
    pub include_children: bool,

    /// Number of random nodes to forward the request to at each step
    pub fanout: u8,

    /// List of Peer IDs the request traversed so far
    pub path: Vec<PeerId>,
}

/// Block request by ID using a random walk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockSearchRandom {
    V0(BlockSearchRandomV0),
}

/// Response to a BlockSearch* request
///
/// Follows request path with possible shortcuts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockResultV0 {
    /// Response path
    pub path: Vec<PeerId>,

    /// Resulting Object(s)
    pub payload: Vec<Block>,
}

/// Response to a BlockSearch* request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockResult {
    V0(BlockResultV0),
}

/// Request latest events corresponding to the branch heads in a pub/sub topic
///
/// In response an Event is sent for each commit chunk that belong to branch heads
/// that are not present in the requestor's known heads
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchHeadsReqV0 {
    /// Topic public key of the branch
    pub topic: TopicId,

    /// Known heads
    pub known_heads: Vec<ObjectId>,
}

/// Request latest events corresponding to the branch heads in a pub/sub topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BranchHeadsReq {
    V0(BranchHeadsReqV0),
}

/// Branch synchronization request
///
/// In response a stream of `Block`s of the requested Objects are sent
/// that are not present in the requestor's known heads and commits
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchSyncReqV0 {
    /// Heads to request, including all their dependencies
    pub heads: Vec<ObjectId>,

    /// Fully synchronized until these commits
    pub known_heads: Vec<ObjectId>,

    /// Known commit IDs since known_heads
    pub known_commits: BloomFilter,
}

/// Branch synchronization request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BranchSyncReq {
    V0(BranchSyncReqV0),
}

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

/// Events the responder has, see EventRespV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HaveEventsV0 {
    /// Publisher ID
    pub publisher: Digest,

    /// First sequence number to send
    pub from: u32,

    /// Last sequence number to send
    pub to: u32,
}

/// Request missed events for a pub/sub topic
/// for the specified range of publisher sequence numbers
///
/// In response an EventResp then a stream of Events are sent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventReqV0 {
    /// Topic public key
    pub topic: TopicId,

    /// Events needed by the requestor
    pub need: Vec<NeedEventsV0>,
}

/// Request missed events for a pub/sub topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventReq {
    V0(EventReqV0),
}

/// Response to an EventReq
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventRespV0 {
    /// Events the responder has
    pub have: Vec<HaveEventsV0>,
}

/// Response to an EventReq
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventResp {
    V0(EventRespV0),
}

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
    Block(Block),
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

/// Content of PeerAdvertV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerAdvertContentV0 {
    /// Peer ID
    pub peer: PeerId,

    /// Topic subscriptions
    pub subs: BloomFilter128,

    /// Network addresses
    pub address: Vec<NetAddr>,

    /// Version number
    pub version: u16,

    /// App-specific metadata (profile, cryptographic material, etc)
    #[serde(with = "serde_bytes")]
    pub metadata: Vec<u8>,
}

/// Peer advertisement
///
/// Sent periodically across the overlay along random walks.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerAdvertV0 {
    /// Peer advertisement content
    pub content: PeerAdvertContentV0,

    /// Signature over content by peer's private key
    pub sig: Sig,

    /// Time-to-live, decremented at each hop
    pub ttl: u8,
}

/// Peer advertisement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PeerAdvert {
    V0(PeerAdvertV0),
}

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
    BlockSearchTopic(BlockSearchTopic),
    BlockSearchRandom(BlockSearchRandom),
    BlockResult(BlockResult),
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
// BROKER PROTOCOL
//

/// Content of AddUserV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AddUserContentV0 {
    /// User pub key
    pub user: PubKey,
}

/// Add user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AddUserV0 {
    pub content: AddUserContentV0,

    /// Signature by admin key
    pub sig: Sig,
}

/// Add user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AddUser {
    V0(AddUserV0),
}

impl AddUser {
    pub fn content_v0(&self) -> AddUserContentV0 {
        match self {
            AddUser::V0(o) => o.content,
        }
    }
    pub fn sig(&self) -> Sig {
        match self {
            AddUser::V0(o) => o.sig,
        }
    }
    pub fn user(&self) -> PubKey {
        match self {
            AddUser::V0(o) => o.content.user,
        }
    }
}

/// Content of DelUserV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DelUserContentV0 {
    /// User pub key
    pub user: PubKey,
}

/// Delete user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DelUserV0 {
    pub content: DelUserContentV0,

    /// Signature by admin key
    pub sig: Sig,
}

/// Delete user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DelUser {
    V0(DelUserV0),
}

impl DelUser {
    pub fn content_v0(&self) -> DelUserContentV0 {
        match self {
            DelUser::V0(o) => o.content,
        }
    }
    pub fn sig(&self) -> Sig {
        match self {
            DelUser::V0(o) => o.sig,
        }
    }
    pub fn user(&self) -> PubKey {
        match self {
            DelUser::V0(o) => o.content.user,
        }
    }
}

/// Content of `AddClientV0`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AddClientContentV0 {
    /// Client pub key
    pub client: PubKey,
}
/// Add a client
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AddClientV0 {
    pub content: AddClientContentV0,

    /// Signature by user key
    pub sig: Sig,
}

/// Add a client
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AddClient {
    V0(AddClientV0),
}

impl AddClient {
    pub fn content_v0(&self) -> AddClientContentV0 {
        match self {
            AddClient::V0(o) => o.content,
        }
    }
    pub fn sig(&self) -> Sig {
        match self {
            AddClient::V0(o) => o.sig,
        }
    }
    pub fn client(&self) -> PubKey {
        match self {
            AddClient::V0(o) => o.content.client,
        }
    }
}

/// Content of `DelClientV0`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DelClientContentV0 {
    /// Client pub key
    pub client: PubKey,
}

/// Remove a client
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DelClientV0 {
    pub content: DelClientContentV0,

    /// Signature by user key
    pub sig: Sig,
}

/// Remove a client
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DelClient {
    V0(DelClientV0),
}

impl DelClient {
    pub fn content_v0(&self) -> DelClientContentV0 {
        match self {
            DelClient::V0(o) => o.content,
        }
    }
    pub fn sig(&self) -> Sig {
        match self {
            DelClient::V0(o) => o.sig,
        }
    }
    pub fn client(&self) -> PubKey {
        match self {
            DelClient::V0(o) => o.content.client,
        }
    }
}

/// Content of `BrokerRequestV0`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerRequestContentV0 {
    AddUser(AddUser),
    DelUser(DelUser),
    AddClient(AddClient),
    DelClient(DelClient),
}
/// Broker request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerRequestV0 {
    /// Request ID
    pub id: u64,

    /// Request content
    pub content: BrokerRequestContentV0,
}

/// Broker request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerRequest {
    V0(BrokerRequestV0),
}

impl BrokerRequest {
    pub fn id(&self) -> u64 {
        match self {
            BrokerRequest::V0(o) => o.id,
        }
    }
    pub fn content_v0(&self) -> BrokerRequestContentV0 {
        match self {
            BrokerRequest::V0(o) => o.content.clone(),
        }
    }
}

/// Response to a `BrokerRequest`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result (including but not limited to Result)
    pub result: u16,
}

/// Response to a `BrokerRequest`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerResponse {
    V0(BrokerResponseV0),
}

impl BrokerResponse {
    pub fn id(&self) -> u64 {
        match self {
            BrokerResponse::V0(o) => o.id,
        }
    }
    pub fn result(&self) -> u16 {
        match self {
            BrokerResponse::V0(o) => o.result,
        }
    }
}

/// Request to join an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayJoinV0 {
    /// Overlay secret
    pub secret: SymKey,

    /// Repository the overlay belongs to.
    /// Only set for local brokers.
    pub repo_pubkey: Option<PubKey>,

    /// Secret for the repository.
    /// Only set for local brokers.
    pub repo_secret: Option<SymKey>,

    /// Peers to connect to
    pub peers: Vec<PeerAdvert>,
}

/// Request to join an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayJoin {
    V0(OverlayJoinV0),
}

impl OverlayJoin {
    pub fn secret(&self) -> SymKey {
        match self {
            OverlayJoin::V0(o) => o.secret,
        }
    }
    pub fn peers(&self) -> &Vec<PeerAdvert> {
        match self {
            OverlayJoin::V0(o) => &o.peers,
        }
    }
}

/// Request to leave an overlay
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OverlayLeave {
    V0(),
}

/// Request a Block by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockGetV0 {
    /// Block ID to request
    pub id: BlockId,

    /// Whether or not to include all children recursively
    pub include_children: bool,

    /// Topic the object is referenced from
    pub topic: Option<PubKey>,
}

/// Request an object by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockGet {
    V0(BlockGetV0),
}

/// Request to store an object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockPut {
    V0(Block),
}

impl BlockPut {
    pub fn block(&self) -> &Block {
        match self {
            BlockPut::V0(o) => &o,
        }
    }
}

/// Request to pin an object
///
/// Brokers maintain an LRU cache of objects,
/// where old, unused objects might get deleted to free up space for new ones.
/// Pinned objects are retained, regardless of last access.
/// Note that expiry is still observed in case of pinned objects.
/// To make an object survive its expiry,
/// it needs to be copied with a different expiry time.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ObjectPinV0 {
    pub id: ObjectId,
}

/// Request to pin an object
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ObjectPin {
    ObjectPinV0,
}

/// Request to unpin an object
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ObjectUnpinV0 {
    pub id: ObjectId,
}

/// Request to unpin an object
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ObjectUnpin {
    V0(ObjectUnpinV0),
}

/// Request to copy an object with a different expiry time
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ObjectCopyV0 {
    /// Object ID to copy
    pub id: ObjectId,

    /// New expiry time
    pub expiry: Option<Timestamp>,
}

/// Request to copy an object with a different expiry time
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ObjectCopy {
    V0(ObjectCopyV0),
}

/// Request to delete an object
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ObjectDelV0 {
    pub id: ObjectId,
}

/// Request to delete an object
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ObjectDel {
    V0(ObjectDelV0),
}

/// Request subscription to a `Topic`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicSubV0 {
    /// Topic to subscribe
    pub topic: PubKey,

    /// Publisher need to prived a signed `TopicAdvert` for the PeerId of the broker
    pub advert: Option<TopicAdvert>,
}

/// Request subscription to a `Topic`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicSub {
    V0(TopicSubV0),
}

/// Request unsubscription from a `Topic`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicUnsubV0 {
    /// Topic to unsubscribe
    pub topic: PubKey,
}

/// Request unsubscription from a `Topic`
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicUnsub {
    V0(TopicUnsubV0),
}

/// Connect to an already subscribed `Topic`, and start receiving its `Event`s
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicConnectV0 {
    /// Topic to connect
    pub topic: PubKey,
}

/// Connect to an already subscribed `Topic`, and start receiving its `Event`s
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicConnect {
    V0(TopicConnectV0),
}

/// Disconnect from a Topic, and stop receiving its `Event`s
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicDisconnectV0 {
    /// Topic to disconnect
    pub topic: PubKey,
}

/// Disconnect from a Topic, and stop receiving its `Event`s
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicDisconnect {
    V0(TopicDisconnectV0),
}

/// Content of `BrokerOverlayRequestV0`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayRequestContentV0 {
    OverlayConnect(OverlayConnect),
    OverlayDisconnect(OverlayDisconnect),
    OverlayJoin(OverlayJoin),
    OverlayLeave(OverlayLeave),
    TopicSub(TopicSub),
    TopicUnsub(TopicUnsub),
    TopicConnect(TopicConnect),
    TopicDisconnect(TopicDisconnect),
    Event(Event),
    BlockGet(BlockGet),
    BlockPut(BlockPut),
    ObjectPin(ObjectPin),
    ObjectUnpin(ObjectUnpin),
    ObjectCopy(ObjectCopy),
    ObjectDel(ObjectDel),
    BranchHeadsReq(BranchHeadsReq),
    BranchSyncReq(BranchSyncReq),
}
/// Broker overlay request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerOverlayRequestV0 {
    /// Request ID
    pub id: u64,

    /// Request content
    pub content: BrokerOverlayRequestContentV0,
}

/// Broker overlay request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayRequest {
    V0(BrokerOverlayRequestV0),
}

impl BrokerOverlayRequest {
    pub fn id(&self) -> u64 {
        match self {
            BrokerOverlayRequest::V0(o) => o.id,
        }
    }
    pub fn content_v0(&self) -> &BrokerOverlayRequestContentV0 {
        match self {
            BrokerOverlayRequest::V0(o) => &o.content,
        }
    }
}

/// Content of `BrokerOverlayResponseV0`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayResponseContentV0 {
    Block(Block),
}

/// Response to a `BrokerOverlayRequest`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerOverlayResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result (including but not limited to Result)
    pub result: u16,

    /// Response content
    pub content: Option<BrokerOverlayResponseContentV0>,
}

/// Response to a `BrokerOverlayRequest`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayResponse {
    V0(BrokerOverlayResponseV0),
}

impl BrokerOverlayResponse {
    pub fn id(&self) -> u64 {
        match self {
            BrokerOverlayResponse::V0(o) => o.id,
        }
    }
    pub fn result(&self) -> u16 {
        match self {
            BrokerOverlayResponse::V0(o) => o.result,
        }
    }
    pub fn block(&self) -> Option<&Block> {
        match self {
            BrokerOverlayResponse::V0(o) => match &o.content {
                Some(contentv0) => match contentv0 {
                    BrokerOverlayResponseContentV0::Block(b) => Some(b),
                },
                None => None,
            },
        }
    }
}

/// Content of `BrokerOverlayMessageV0`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayMessageContentV0 {
    BrokerOverlayRequest(BrokerOverlayRequest),
    BrokerOverlayResponse(BrokerOverlayResponse),
    Event(Event),
}
/// Broker message for an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerOverlayMessageV0 {
    pub overlay: OverlayId,
    pub content: BrokerOverlayMessageContentV0,
}

/// Broker message for an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerOverlayMessage {
    V0(BrokerOverlayMessageV0),
}

impl BrokerOverlayMessage {
    pub fn content_v0(&self) -> &BrokerOverlayMessageContentV0 {
        match self {
            BrokerOverlayMessage::V0(o) => &o.content,
        }
    }
    pub fn overlay_request(&self) -> &BrokerOverlayRequest {
        match self {
            BrokerOverlayMessage::V0(o) => match &o.content {
                BrokerOverlayMessageContentV0::BrokerOverlayRequest(r) => &r,
                _ => panic!("not an overlay request"),
            },
        }
    }
    pub fn overlay_id(&self) -> OverlayId {
        match self {
            BrokerOverlayMessage::V0(o) => o.overlay,
        }
    }
    pub fn is_request(&self) -> bool {
        match self {
            BrokerOverlayMessage::V0(o) => matches!(
                o.content,
                BrokerOverlayMessageContentV0::BrokerOverlayRequest { .. }
            ),
        }
    }
    pub fn is_response(&self) -> bool {
        match self {
            BrokerOverlayMessage::V0(o) => matches!(
                o.content,
                BrokerOverlayMessageContentV0::BrokerOverlayResponse { .. }
            ),
        }
    }
    pub fn id(&self) -> u64 {
        match self {
            BrokerOverlayMessage::V0(o) => match &o.content {
                BrokerOverlayMessageContentV0::BrokerOverlayResponse(r) => r.id(),
                BrokerOverlayMessageContentV0::BrokerOverlayRequest(r) => r.id(),
                BrokerOverlayMessageContentV0::Event(_) => {
                    panic!("it is an event")
                }
            },
        }
    }
    pub fn result(&self) -> u16 {
        match self {
            BrokerOverlayMessage::V0(o) => match &o.content {
                BrokerOverlayMessageContentV0::BrokerOverlayResponse(r) => r.result(),
                BrokerOverlayMessageContentV0::BrokerOverlayRequest(r) => {
                    panic!("it is not a response");
                }
                BrokerOverlayMessageContentV0::Event(_) => {
                    panic!("it is not a response");
                }
            },
        }
    }
    pub fn block<'a>(&self) -> Option<&Block> {
        match self {
            BrokerOverlayMessage::V0(o) => match &o.content {
                BrokerOverlayMessageContentV0::BrokerOverlayResponse(r) => r.block(),
                BrokerOverlayMessageContentV0::BrokerOverlayRequest(r) => {
                    panic!("it is not a response");
                }
                BrokerOverlayMessageContentV0::Event(_) => {
                    panic!("it is not a response");
                }
            },
        }
    }
}

/// Content of BrokerMessageV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerMessageContentV0 {
    BrokerRequest(BrokerRequest),
    BrokerResponse(BrokerResponse),
    BrokerOverlayMessage(BrokerOverlayMessage),
}

/// Broker message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrokerMessageV0 {
    /// Message content
    pub content: BrokerMessageContentV0,

    /// Optional padding
    #[serde(with = "serde_bytes")]
    pub padding: Vec<u8>,
}

/// Broker message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BrokerMessage {
    V0(BrokerMessageV0),
}

impl BrokerMessage {
    /// Get the content
    pub fn content(&self) -> BrokerMessageContentV0 {
        match self {
            BrokerMessage::V0(o) => o.content.clone(),
        }
    }
    pub fn is_request(&self) -> bool {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => p.is_request(),
                BrokerMessageContentV0::BrokerResponse(_) => false,
                BrokerMessageContentV0::BrokerRequest(_) => true,
            },
        }
    }
    pub fn is_response(&self) -> bool {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => p.is_response(),
                BrokerMessageContentV0::BrokerResponse(_) => true,
                BrokerMessageContentV0::BrokerRequest(_) => false,
            },
        }
    }
    pub fn id(&self) -> u64 {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => p.id(),
                BrokerMessageContentV0::BrokerResponse(r) => r.id(),
                BrokerMessageContentV0::BrokerRequest(r) => r.id(),
            },
        }
    }
    pub fn result(&self) -> u16 {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => p.result(),
                BrokerMessageContentV0::BrokerResponse(r) => r.result(),
                BrokerMessageContentV0::BrokerRequest(_) => {
                    panic!("it is not a response");
                }
            },
        }
    }
    pub fn is_overlay(&self) -> bool {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => true,
                BrokerMessageContentV0::BrokerResponse(r) => false,
                BrokerMessageContentV0::BrokerRequest(r) => false,
            },
        }
    }
    pub fn response_block(&self) -> Option<&Block> {
        match self {
            BrokerMessage::V0(o) => match &o.content {
                BrokerMessageContentV0::BrokerOverlayMessage(p) => p.block(),
                BrokerMessageContentV0::BrokerResponse(r) => {
                    panic!("it doesn't have a response block. it is not an overlay response");
                }
                BrokerMessageContentV0::BrokerRequest(_) => {
                    panic!("it is not a response");
                }
            },
        }
    }
}

//
// EXTERNAL REQUESTS
//

/// Request object(s) by ID from a repository by non-members
///
/// The request is sent by a non-member to an overlay member node,
/// which has a replica of the repository.
///
/// The response includes the requested objects and all their children recursively,
/// and optionally all object dependencies recursively.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtObjectGetV0 {
    /// Repository to request the objects from
    pub repo: PubKey,

    /// List of Object IDs to request, including their children
    pub ids: Vec<ObjectId>,

    /// Whether or not to include all children recursively
    pub include_children: bool,

    /// Expiry time after which the link becomes invalid
    pub expiry: Option<Timestamp>,
}

/// Request object(s) by ID from a repository by non-members
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtObjectGet {
    V0(ExtObjectGetV0),
}

/// Branch heads request
pub type ExtBranchHeadsReq = BranchHeadsReq;

/// Branch synchronization request
pub type ExtBranchSyncReq = BranchSyncReq;

/// Content of ExtRequestV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtRequestContentV0 {
    ExtObjectGet(ExtObjectGet),
    ExtBranchHeadsReq(ExtBranchHeadsReq),
    ExtBranchSyncReq(ExtBranchSyncReq),
}

/// External request authenticated by a MAC
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtRequestV0 {
    /// Request ID
    pub id: u64,

    /// Request content
    pub content: ExtRequestContentV0,

    /// BLAKE3 MAC over content
    /// BLAKE3 keyed hash:
    /// - key: BLAKE3 derive_key ("LoFiRe ExtRequest BLAKE3 key",
    ///                           repo_pubkey + repo_secret)
    pub mac: Digest,
}

/// External request authenticated by a MAC
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtRequest {
    V0(ExtRequestV0),
}

/// Content of ExtResponseV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtResponseContentV0 {
    Block(Block),
    EventResp(EventResp),
    Event(Event),
}

/// Response to an ExtRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result code
    pub result: u16,

    /// Response content
    pub content: Option<ExtResponseContentV0>,
}

/// Response to an ExtRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtResponse {
    V0(ExtResponseV0),
}

///
/// AUTHENTICATION MESSAGES
///

/// Client Hello
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientHello {
    V0(),
}

/// Start chosen protocol
/// First message sent by the client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StartProtocol {
    Auth(ClientHello),
    Ext(ExtRequest),
}

/// Server hello sent upon a client connection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerHelloV0 {
    /// Nonce for ClientAuth
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
}

/// Server hello sent upon a client connection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerHello {
    V0(ServerHelloV0),
}

impl ServerHello {
    pub fn nonce(&self) -> &Vec<u8> {
        match self {
            ServerHello::V0(o) => &o.nonce,
        }
    }
}

/// Content of ClientAuthV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientAuthContentV0 {
    /// User pub key
    pub user: PubKey,

    /// Client pub key
    pub client: PubKey,

    /// Nonce from ServerHello
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
}

/// Client authentication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientAuthV0 {
    /// Authentication data
    pub content: ClientAuthContentV0,

    /// Signature by client key
    pub sig: Sig,
}

/// Client authentication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientAuth {
    V0(ClientAuthV0),
}

impl ClientAuth {
    pub fn content_v0(&self) -> ClientAuthContentV0 {
        match self {
            ClientAuth::V0(o) => o.content.clone(),
        }
    }
    pub fn sig(&self) -> Sig {
        match self {
            ClientAuth::V0(o) => o.sig,
        }
    }
    pub fn user(&self) -> PubKey {
        match self {
            ClientAuth::V0(o) => o.content.user,
        }
    }
    pub fn client(&self) -> PubKey {
        match self {
            ClientAuth::V0(o) => o.content.client,
        }
    }
    pub fn nonce(&self) -> &Vec<u8> {
        match self {
            ClientAuth::V0(o) => &o.content.nonce,
        }
    }
}

/// Authentication result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResultV0 {
    pub result: u16,
    #[serde(with = "serde_bytes")]
    pub metadata: Vec<u8>,
}

/// Authentication result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthResult {
    V0(AuthResultV0),
}

impl AuthResult {
    pub fn result(&self) -> u16 {
        match self {
            AuthResult::V0(o) => o.result,
        }
    }
    pub fn metadata(&self) -> &Vec<u8> {
        match self {
            AuthResult::V0(o) => &o.metadata,
        }
    }
}

//
// DIRECT / OUT-OF-BAND MESSAGES
//

/// Link/invitation to the repository
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoLinkV0 {
    /// Repository public key ID
    pub id: PubKey,

    /// Repository secret
    pub secret: SymKey,

    /// Peers to connect to
    pub peers: Vec<PeerAdvert>,
}

/// Link/invitation to the repository
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RepoLink {
    V0(RepoLinkV0),
}

impl RepoLink {
    pub fn id(&self) -> PubKey {
        match self {
            RepoLink::V0(o) => o.id,
        }
    }
    pub fn secret(&self) -> SymKey {
        match self {
            RepoLink::V0(o) => o.secret,
        }
    }
    pub fn peers(&self) -> Vec<PeerAdvert> {
        match self {
            RepoLink::V0(o) => o.peers.clone(),
        }
    }
}

/// Link to object(s) or to a branch from a repository
/// that can be shared to non-members
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectLinkV0 {
    /// Request to send to an overlay peer
    pub req: ExtRequest,

    /// Keys for the root blocks of the requested objects
    pub keys: Vec<ObjectRef>,
}

/// Link to object(s) or to a branch from a repository
/// that can be shared to non-members
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectLink {
    V0(ObjectLinkV0),
}

/// Owned repository with private key
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoKeysV0 {
    /// Repository private key
    pub key: PrivKey,

    /// Repository secret
    pub secret: SymKey,

    /// Peers to connect to
    pub peers: Vec<PeerAdvert>,
}

/// Owned repository with private key
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RepoKeys {
    V0(RepoKeysV0),
}
