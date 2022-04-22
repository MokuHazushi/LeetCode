// https://leetcode.com/problems/design-hashmap/

// This HashMap uses dynamic memory and is fully
// reallocated when the capacity of all the buckets
// has been reached
struct MyHashMap {
    buckets: Vec<Vec<Entry>>,
    nb_buckets: usize,
    bucket_size: usize,
    nb_stored_elements: usize,
}

struct Entry {
    value: i32,
    key: i32,
}


impl MyHashMap {

    fn new() -> Self {
        MyHashMap::allocate(1000, 10)
    }
    
    // O(bucket_size)
    fn put(&mut self, key: i32, value: i32) {
        
        match self.find_element(key) {
            Some(mut entry) => entry.value = value,
            None => {
                if self.nb_stored_elements+1 >= self.nb_buckets*self.bucket_size {
                    *self = self.reallocate();
                }

                if let Some(bucket) = self.buckets.get_mut(key as usize % self.nb_buckets) {
                    let entry = Entry{value: value, key: key};
                    bucket.push(entry);
                    self.nb_stored_elements += 1;
                }
            }
        }
    }

    // O(bucket_size)
    fn get(&mut self, key: i32) -> i32 {
        match self.find_element(key) {
                Some(entry) => entry.value,
                None => -1,
        }
    }
    
    // O(bucket_size)
    fn remove(&mut self, key: i32) {
        if let Some(bucket) = self.buckets.get_mut(key as usize % self.nb_buckets) {
            let mut index = -1;
            for i in 0..bucket.len() {
                if bucket[i].key == key {
                    index = i as i32;
                    break;
                }
            }        

            if index != -1 {
                bucket.swap_remove(index as usize);
            }
        }
    }

    fn allocate(nb_buckets: usize, bucket_size: usize) -> Self {
        let mut hashmap = MyHashMap {
            buckets: Vec::new(),
            nb_buckets: nb_buckets,
            bucket_size: bucket_size,
            nb_stored_elements: 0,
        };
        hashmap.buckets.reserve(hashmap.nb_buckets);
        for _ in 0..hashmap.nb_buckets {
            let mut bucket = Vec::new();
            bucket.reserve(hashmap.bucket_size);
            hashmap.buckets.push(bucket);
        }
        hashmap
    }

    // O(nb_stored_elements * bucket_size)
    fn reallocate(&self) -> Self {
        let mut hashmap = MyHashMap::allocate(self.nb_buckets*10, self.bucket_size);

        for bucket in &self.buckets {
            for entry in bucket {
                hashmap.put(entry.key, entry.value);
            }
        }
        hashmap
    }

    // O(bucket_size)
    fn find_element(&mut self, key: i32) -> Option<&mut Entry> {
        if let Some(bucket) = self.buckets.get_mut(key as usize % self.nb_buckets) {
            for entry in bucket {
                if entry.key == key {
                    return Some(entry);
                }
            }
            return None;
        }
        None
    }
}

fn main() {
    println!("Implement my own hashmap!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn put_contains_remove_contains_check() {
        let mut hashmap = MyHashMap::new();

        for i in 0..500 {
            hashmap.put(i, i);
            assert_eq!(hashmap.get(i), i, "Key was added but not found by 'get'");
            hashmap.remove(i);
            assert_eq!(hashmap.get(i), -1, "Key was removed but found by 'get'");
        }
    }

    #[test]
    fn reallocate_test() {
        let mut hashmap = MyHashMap::new();

        for i in 0..10000 {
            hashmap.put(i, i);
            assert_eq!(hashmap.get(i), i);
        }
    }

    #[test]
    fn replace_value_test() {
        let mut hashmap = MyHashMap::new();

        for i in 0..500 {
            hashmap.put(0, i);
            assert_eq!(hashmap.get(0), i);
        }
    }

}
