
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EntityKind {
    Unknown     = 0x00,
    WriterWKey  = 0x02,
    WriterNoKey = 0x03,
    ReaderNoKey = 0x04,
    ReaderWKey  = 0x07,
    WriterGroup = 0x08,
    ReaderGroup = 0x09,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Protocol
{
    RTPS = 0x52545053
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntityId
{
    pub key: [u8;3],
    pub kind: EntityKind
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GuidPrefix
{
    vendor: u16,
    prefix: [u8;10]   
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Guid
{
    prefix: GuidPrefix,
    entity_id: EntityId
}

pub const VENDOR_ID: u16 = 0x1E00;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RtpsHdr
{
    protocol: Protocol,
    version: [u8;2],
    vendor: u16,
    guid: Guid
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SubmessageHdr
{
    pub id: u8,
    pub flags: u8,
    pub octent_count: u16,
}

/// A simplified DATA submessage that conveys a data sample.
/// In a complete implementation there would be fields for inline QoS and the serialized payload.
/// Here we include a few key fields:
///
/// - `extra_flags` and `octets_to_inline_qos` for alignment and optional QoS,
/// - `reader_id` and `writer_id` (each 4 bytes) identify the involved endpoints,
/// - `writer_sn` is the sequence number of the sample.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DataSubmessage {
    pub header: SubmessageHdr,
    pub extra_flags: u16,
    pub octets_to_inline_qos: u16,
    pub reader_id: EntityId, // EntityId_t for the target reader
    pub writer_id: EntityId, // EntityId_t for the sending writer
    pub writer_sn: u64,     // SequenceNumber_t (64 bits)
    // Followed by optional inline QoS and the serialized payload.
}
