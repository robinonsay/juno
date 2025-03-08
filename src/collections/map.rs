use core::hash::{Hash, Hasher};
use crate::{error::Error, hash::DjB2Hasher, sformat, string::SString, ERROR_LEN};


pub struct Map<const C: usize, K: Hash + PartialEq, V>
{
    keys: [Option<K>; C],
    values: [Option<V>; C],

}

impl<const C: usize, K: Hash + PartialEq, V> Map<C,K,V>
{
    pub fn new() -> Self
    {
        Self
        {
            keys: [const { None }; C],
            values: [const { None }; C],
        }
    }

    fn find_index(&self, key: &K) -> Result<usize, Error>
    {
        // Get the hash
        let mut hasher = DjB2Hasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        // Calculate the start index
        let index = hash % C;
        // Iterate over table starting at index
        // to find the key
        for i in index..index+C
        {
            // Get the key at the current index
            let curr_key = match &self.keys[i % C]{
                // If there is a key, retrieve it
                Some(k) => k,
                // If there is no key, return the index
                None => return Ok(i)
            };
            // If the current key equals the key return the index
            if *key == *curr_key
            {
                return Ok(i);
            }
        }
        // If the table is full return none
        return Err(Error::CollectionsError(sformat!(ERROR_LEN, "Map is full")));
    }

    pub fn get(&self, key: &K) -> Result<Option<&V>, Error>
    {
        let index = self.find_index(key)?;
        let value = match &self.values[index]
        {
            Some(value) => value,
            None => return Ok(None)
        };
        return Ok(Some(value));
    }

    pub fn get_mut(&mut self, key: &K) -> Result<Option<&mut V>, Error>
    {

        let index = self.find_index(key)?;
        let value = match &mut self.values[index]
        {
            Some(value) => value,
            None => return Ok(None)
        };
        return Ok(Some(value));
    }

    pub fn set(&mut self, key: K, value: V) -> Result<(), Error>
    {

        let index = self.find_index(&key)?;
        self.keys[index] = Some(key);
        self.values[index] = Some(value);
        return Ok(());
    }
    // pub fn get_value
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_set_get() {
            const C: usize = 4;
            let mut map = Map::<C, i32, i32>::new();
            // get on an empty map returns None.
            assert_eq!(map.get(&10).unwrap(), None);

            // Inserting a key-value pair.
            map.set(10, 100).unwrap();
            // Retrieving returns the correct value.
            assert_eq!(map.get(&10).unwrap(), Some(&100));
        }

        #[test]
        fn test_get_mut_update() {
            const C: usize = 4;
            let mut map = Map::<C, i32, i32>::new();
            map.set(20, 200).unwrap();
            // Use get_mut to modify the value.
            if let Some(value) = map.get_mut(&20).unwrap() {
                *value = 250;
            }
            assert_eq!(map.get(&20).unwrap(), Some(&250));
        }

        #[test]
        fn test_non_existent_key() {
            const C: usize = 4;
            let mut map = Map::<C, i32, i32>::new();
            map.set(30, 300).unwrap();
            // Requesting an unknown key returns Ok(None)
            assert_eq!(map.get(&40).unwrap(), None);
        }

        #[test]
        fn test_map_full() {
            // Use a small capacity map to force a full table.
            const C: usize = 2;
            let mut map = Map::<C, i32, i32>::new();
            // Fill the map.
            map.set(1, 10).unwrap();
            map.set(2, 20).unwrap();
            // Inserting a new key should result in an error.
            let err = map.set(3, 30).unwrap_err();
            match err {
                Error::CollectionsError(_) => {
                    
                },
            }
        }
    }
}
