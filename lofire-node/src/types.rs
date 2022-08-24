//! LoFiRe node data types
//!
//! Corresponds to the BARE schema

use lofire::types::*;
use lofire_repo::types::*;
use serde::{Deserialize, Serialize};

//
// COMMON DATA TYPES FOR MESSAGES
//

/// Peer ID: public key of node
pub type PeerId = PubKey;

/// IPv4 address
pub type IPv4 = [u8; 4];

/// IPv6 address
pub type IPv6 = [u8; 16];

/// IP address
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum IP {
    IPv4(IPv4),
    IPv6(IPv6),
}

/// IP transport protocol
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum IPTransportProtocol {
    TLS,
    QUIC,
}

/// IP transport address
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IPTransportAddr {
    pub ip: IP,
    pub port: u16,
    pub protocol: IPTransportProtocol,
}

/// Network address
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NetAddr {
    IPTransport(IPTransportAddr),
}

/// Bloom filter (variable size)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BloomFilter {
    /// Number of hash functions
    pub k: u8,

    /// Filter
    pub f: Vec<u8>,
}

/// Bloom filter (128 B)
///
/// (m=1024; k=7; p=0.01; n=107)
pub type BloomFilter128 = [[u8; 32]; 4];

/// Bloom filter (1 KiB)
///
/// (m=8192; k=7; p=0.01; n=855)
pub type BloomFilter1K = [[u8; 32]; 32];

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

/// Branch change notification
/// Contains a chunk of a newly added Commit or File referenced by a commit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeV0 {
    /// Object with encrypted content
    pub content: Object,

    /// Encrypted key for the Commit object in content
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
    pub topic: PubKey,

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
    pub sig: Signature,
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

/// Request latest events corresponding to the branch heads in a pub/sub topic
///
/// In response an Event is sent for each commit chunk that belong to branch heads
/// that are not present in the requestor's known heads
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchHeadsReqV0 {
    /// Topic public key of the branch
    pub topic: PubKey,

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
/// In response a stream of Objects are sent
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
    pub topic: PubKey,

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
    pub sig: Signature,

    /// Time-to-live, decremented at each hop
    pub ttl: u8,
}

/// Peer advertisement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PeerAdvert {
    V0(PeerAdvertV0),
}

/// Overlay ID
///
/// - for public overlays that need to be discovered by public key:
///   BLAKE3 hash over the repository public key
/// - for private overlays:
///   BLAKE3 keyed hash over the repository public key
///   - key: BLAKE3 derive_key ("LoFiRe OverlayId BLAKE3 key", repo_secret)
type OverlayId = Digest;

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
// BROKER MESSAGES
//

/// Server hello sent upon a client connection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerHelloV0 {
    /// Nonce for ClientAuth
    pub nonce: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerHello {
    V0(ServerHelloV0),
}

/// Content of ClientAuthV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientAuthContentV0 {
    /// User pub key
    pub user: PubKey,

    /// Device pub key
    pub device: PubKey,

    /// Nonce from ServerHello
    pub nonce: Vec<u8>,
}

/// Client authentication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientAuthV0 {
    /// Authentication data
    pub content: ClientAuthContentV0,

    /// Signature by device key
    pub sig: Signature,
}

/// Client authentication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientAuth {
    V0(ClientAuthV0),
}

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
    pub sig: Signature,
}

/// Add user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AddUser {
    V0(AddUserV0),
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
    pub sig: Signature,
}

/// Delete user account
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DelUser {
    V0(DelUserV0),
}

/// Content of AuthorizeDeviceKeyV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AuthorizeDeviceKeyContentV0 {
    /// Device pub key
    pub device: PubKey,
}
/// Authorize device key
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AuthorizeDeviceKeyV0 {
    pub content: AuthorizeDeviceKeyContentV0,

    /// Signature by user key
    pub sig: Signature,
}

/// Authorize device key
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AuthorizeDeviceKey {
    V0(AuthorizeDeviceKeyV0),
}

/// Content of RevokeDeviceKeyV0
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RevokeDeviceKeyContentV0 {
    /// Device pub key
    pub device: PubKey,
}

/// Revoke device key
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RevokeDeviceKeyV0 {
    pub content: RevokeDeviceKeyContentV0,

    /// Signature by user key
    pub sig: Signature,
}

/// Revoke device key
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RevokeDeviceKey {
    V0(RevokeDeviceKeyV0),
}

/// Request to join an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayJoinV0 {
    /// Overlay secret
    pub secret: SymKey,

    /// Peers to connect to
    pub peers: Vec<PeerAdvert>,
}

/// Request to join an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayJoin {
    V0(OverlayJoinV0),
}

/// Request to leave an overlay
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OverlayLeave {
    V0(),
}

/// Request an object by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectGetV0 {
    /// List of Object IDs to request
    pub ids: Vec<ObjectId>,

    /// Whether or not to include all children recursively
    pub include_children: bool,

    /// Topic the object is referenced from
    pub topic: Option<PubKey>,
}

/// Request an object by ID
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectGet {
    V0(ObjectGetV0),
}

/// Request to store an object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectPutV0 {
    pub object: Object,
}

/// Request to store an object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectPut {
    V0(ObjectPutV0),
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

/// Request subscription to a topic
///
/// For publishers a private key also needs to be provided.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicSubV0 {
    /// Topic to subscribe
    pub topic: PubKey,

    /// Topic private key for publishers
    pub key: Option<PrivKey>,
}

/// Request subscription to a topic
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicSub {
    V0(TopicSubV0),
}

/// Request unsubscription from a topic
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TopicUnsubV0 {
    /// Topic to unsubscribe
    pub topic: PubKey,
}

/// Request unsubscription from a topic
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TopicUnsub {
    V0(TopicUnsubV0),
}

/// Content of AppRequestV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppRequestContentV0 {
    OverlayJoin(OverlayJoin),
    OverlayLeave(OverlayLeave),
    TopicSub(TopicSub),
    TopicUnsub(TopicUnsub),
    Event(Event),
    ObjectGet(ObjectGet),
    ObjectPut(ObjectPut),
    ObjectPin(ObjectPin),
    ObjectUnpin(ObjectUnpin),
    ObjectCopy(ObjectCopy),
    ObjectDel(ObjectDel),
    BranchHeadsReq(BranchHeadsReq),
    BranchSyncReq(BranchSyncReq),
}
/// Application request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppRequestV0 {
    /// Request ID
    pub id: u64,

    /// Request content
    pub content: AppRequestContentV0,
}

/// Application request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppRequest {
    V0(AppRequestV0),
}

/// Result codes
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Result {
    Ok,
    Error,
}

/// Content of AppResponseV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppResponseContentV0 {
    Object(Object),
}

/// Response to an AppRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result (incl but not limited to Result)
    pub result: u8,
    pub content: Option<AppResponseContentV0>,
}

/// Response to an AppRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppResponse {
    V0(AppResponseV0),
}

/// Content of AppOverlayMessageV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppOverlayMessageContentV0 {
    AppRequest(AppRequest),
    AppResponse(AppResponse),
    Event(Event),
}
/// Application message for an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppOverlayMessageV0 {
    pub overlay: OverlayId,
    pub content: AppOverlayMessageContentV0,
}

/// Application message for an overlay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppOverlayMessage {
    V0(AppOverlayMessageV0),
}

/// Content of AppMessageV0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppMessageContentV0 {
    ServerHello(ServerHello),
    ClientAuth(ClientAuth),
    AddUser(AddUser),
    DelUser(DelUser),
    AuthorizeDeviceKey(AuthorizeDeviceKey),
    RevokeDeviceKey(RevokeDeviceKey),
    AppOverlayMessage(AppOverlayMessage),
}

/// Application message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppMessageV0 {
    /// Message content
    pub content: AppMessageContentV0,

    /// Optional padding
    pub padding: Vec<u8>,
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
    Object(Object),
    EventResp(EventResp),
    Event(Event),
}

/// Response to an ExtRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtResponseV0 {
    /// Request ID
    pub id: u64,

    /// Result code
    pub result: u8,

    /// Response content
    pub content: Option<ExtResponseContentV0>,
}

/// Response to an ExtRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtResponse {
    V0(ExtResponseV0),
}

//
// DIRECT MESSAGES
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

/// Link to object(s) or to a branch from a repository
/// that can be shared to non-members
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectLinkV0 {
    /// Request to send to an overlay peer
    pub req: ExtRequest,

    /// Keys for the requested objects
    pub keys: Vec<ObjectRef>,
}

/// Link to object(s) or to a branch from a repository
/// that can be shared to non-members
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectLink {
    V0(ObjectLinkV0),
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
