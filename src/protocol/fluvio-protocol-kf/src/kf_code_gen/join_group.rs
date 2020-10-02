/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::ProtocolMetadata;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfJoinGroupRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfJoinGroupRequest {
    /// The group identifier.
    pub group_id: String,

    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout
    /// in milliseconds.
    pub session_timeout_ms: i32,

    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin
    /// when rebalancing the group.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub rebalance_timeout_ms: i32,

    /// The member id assigned by the group coordinator.
    pub member_id: String,

    /// The unique name the for class of protocols implemented by the group we want to join.
    pub protocol_type: String,

    /// The list of protocols that the member supports.
    pub protocols: Vec<JoinGroupRequestProtocol>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct JoinGroupRequestProtocol {
    /// The protocol name.
    pub name: String,

    /// The protocol metadata.
    pub metadata: ProtocolMetadata,
}

// -----------------------------------
// KfJoinGroupResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfJoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The generation ID of the group.
    pub generation_id: i32,

    /// The group protocol selected by the coordinator.
    pub protocol_name: String,

    /// The leader of the group.
    pub leader: String,

    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    pub members: Vec<JoinGroupResponseMember>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    pub member_id: String,

    /// The group member metadata.
    pub metadata: ProtocolMetadata,
}

// -----------------------------------
// Implementation - KfJoinGroupRequest
// -----------------------------------

impl Request for KfJoinGroupRequest {
    const API_KEY: u16 = 11;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 4;
    const DEFAULT_API_VERSION: i16 = 4;

    type Response = KfJoinGroupResponse;
}