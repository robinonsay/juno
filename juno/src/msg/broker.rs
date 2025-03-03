use core::time::Duration;

use alloc::{sync::Arc, vec::Vec};
use osafe::{error::Error, ipc::Communicate};

use super::{Address, Msg, Topic};


pub struct Publisher<T: Communicate>
{
    channel: T,
}

struct Subscriber<T: Communicate>
{
    pub topic: Topic,
    pub channel: T,
}

pub struct Broker<T: Communicate>
{
    in_pipe: T,
    out_pipe: T,
    subscribers: Vec<Option<Subscriber<T>>>
}

impl<T: Communicate> Publisher<T>
{
    pub fn new(channel: T) -> Self
    {
        Self
        {
            channel: channel
        }
    }

    pub fn publish<D: Send>(&self, msg: Msg<D>) -> Result<(), Error>
    {
        self.channel.send(msg)?;
        Ok(())
    }

    pub fn try_publish<D: Send>(&self, msg: Msg<D>, timeout: Duration) -> Result<(), Error>
    {
        self.channel.try_send(msg, timeout.as_millis() as i32)?;
        Ok(())
    }
}

impl <T: Communicate> Subscriber<T>
{
    pub fn new(channel: T, topic: Topic, address: Address) -> Self
    {
        Self
        {
            channel: channel,
            topic: topic,
        }
    }

    pub fn recv<D: Send>(&self) -> Result<Msg<D>, Error>
    {
        let msg = self.channel.recv::<Msg<D>>()?;
        Ok(msg)
    }

    pub fn try_recv<D: Send>(&self, timeout: Duration) -> Result<Option<Msg<D>>, Error>
    {
        let msg = self.channel.try_recv::<Msg<D>>(timeout.as_millis() as i32)?;
        Ok(msg)
    }
}

impl<T: Communicate> Broker<T>
{
    pub fn new(in_pipe: T, out_pipe: T) -> Self
    {
        Self
        {
            in_pipe: in_pipe,
            out_pipe: out_pipe,
            subscribers: Vec::new()
        }
    }
}
