use core::time::Duration;

use osafe::{error::Error, ipc::Communicate};

use crate::math::is_prime;

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

pub struct Broker<T: Communicate, const C: usize>
{
    in_pipe: T,
    out_pipe: T,
    subscribers: [Option<Subscriber<T>>;C],
    length: usize
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

impl<T: Communicate, const C: usize> Broker<T, C>
{
    pub fn new(in_pipe: T, out_pipe: T) -> Self
    {
        Self
        {
            in_pipe: in_pipe,
            out_pipe: out_pipe,
            subscribers: [const {None}; C],
            length: 0
        }
    }

    /// Gets the index for a topic and accounts for collisions
    fn get_index(&self, topic: Topic) -> Option<usize>
    {
        // Return none if there is no space available
        if self.length >= C
        {
            return None;
        }
        // Get the index with the first hash function
        let mut index = (topic % C as Topic) as usize;
        // Check if the index is available
        if self.subscribers[index].is_none()
        {
            // Index is available, return
            return Some(index);
        }
        // Get the R value for the new hash functions:
        // h2(key) = R - (key % R)
        let mut r_mod: usize = 1;
        // Find the next smallest prime numbers
        for i in (1..=C).rev()
        {
            if is_prime(i)
            {
                r_mod = i;
                break;
            }
        }
        // Calaculate the step size
        let step = r_mod - (topic as usize % r_mod/2);
        // Find the next available spot in the table
        for _ in 0..C
        {
            // Increment the index by step
            index += step;
            // Check if the spot is available
            if self.subscribers[index].is_none()
            {
                return Some(index);
            }

        }
        return None;
    }

}


#[cfg(test)]
mod tests
{
    use super::*;

    struct TestComm;

    impl Communicate for TestComm
    {
        fn send<T: Send>(&self, _: T) -> Result<(), Error> {
            return Ok(())
        }
    
        fn try_send<T: Send>(&self, _: T, _:i32) -> Result<(), Error> {
            return Ok(())
        }
    
        fn recv<T: Send>(&self) -> Result<T, Error> {
            todo!()
        }
    
        fn try_recv<T: Send>(&self, _: i32) -> Result<Option<T>, Error> {
            todo!()
        }
    }

    #[test]
    fn test_get_index()
    {
        let mut broker =
        Broker::<TestComm, 10>::new(TestComm{}, TestComm{});
        let test_topic: Topic = 100;
        let index_1 = broker.get_index(test_topic).unwrap();
        broker.subscribers[index_1] = Some(Subscriber::new(TestComm{}, test_topic, 100));
        let index_2 = broker.get_index(test_topic * 2).unwrap();
        assert_ne!(index_1, index_2)
    }
}
