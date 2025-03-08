use crate::{string::SString, TOPIC_LEN};

pub type MsgId = u32;

pub struct Msg<T: Send>
{
    pub topic: SString<TOPIC_LEN>,
    pub mid: MsgId,
    pub data: T
}

