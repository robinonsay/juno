pub mod broker;

pub type Topic = u32;
pub type Address = u16;

pub struct Msg<T: Send>
{
    pub topic: Topic,
    pub data: T
}

impl<T: Send> Msg<T>
{
    pub fn new(topic: Topic, data: T) -> Self
    {
        Msg
        {
            topic: topic,
            data: data
        }
    }
}

