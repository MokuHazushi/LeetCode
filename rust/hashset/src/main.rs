// This HashSet uses dynamic memory and is fully
// reallocated when the capacity of all the buckets
// has been reached
struct MyHashSet {
    buckets: Vec<Vec<i32>>,
    nb_buckets: usize,
    bucket_size: usize,
    nb_stored_elements: usize,
}

impl MyHashSet {

    fn new() -> Self {
        MyHashSet::allocate(1000, 10)
    }
    
    // O(bucket_size)
    fn add(&mut self, key: i32) {
        if self.nb_stored_elements+1 >= self.nb_buckets*self.bucket_size {
            *self = self.reallocate();
        }

        let bucket = &mut self.buckets[key as usize % self.nb_buckets];
        if !bucket.contains(&key) {
            bucket.push(key);
            self.nb_stored_elements += 1;
        }
    }
    
    // O(bucket_size)
    fn remove(&mut self, key: i32) {
        match self.findElement(key) {
            Some(index) => {
                self.buckets[key as usize % self.nb_buckets].swap_remove(index);
                self.nb_stored_elements -= 1;
            },
            None => (),
        }
    }
    
    // O(bucket_size)
    fn contains(&self, key: i32) -> bool {
        match self.findElement(key) {
            Some(_) => true,
            None => false,
        }
    }

    fn allocate(nb_buckets: usize, bucket_size: usize) -> Self {
        let mut hashset = MyHashSet {
            buckets: Vec::new(),
            nb_buckets: nb_buckets,
            bucket_size: bucket_size,
            nb_stored_elements: 0,
        };
        hashset.buckets.reserve(hashset.nb_buckets);
        for _ in 0..hashset.nb_buckets {
            let mut bucket = Vec::new();
            bucket.reserve(hashset.bucket_size);
            hashset.buckets.push(bucket);
        }
        hashset
    }

    // O(nb_stored_elements * bucket_size)
    fn reallocate(&self) -> Self {
        let mut hashset = MyHashSet::allocate(self.nb_buckets*10, self.bucket_size);

        for bucket in &self.buckets {
            for key in bucket {
                hashset.add(*key);
            }
        }

        hashset
    }

    // O(bucket_size)
    fn findElement(&self, key: i32) -> Option<usize> {
        let bucket = &self.buckets[key as usize % self.nb_buckets];
        for i in 0..bucket.len() {
            if bucket[i] == key {
                return Some(i);
            }
        }
        None
    }
}

fn main() {
    println!("Implement my own hashset!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_contains_remove_contains_check() {
        let mut hashset = MyHashSet::new();

        for i in 0..500 {
            hashset.add(i);
            assert_eq!(hashset.contains(i), true, "Key was added but not found by 'contains'");
            hashset.remove(i);
            assert_eq!(hashset.contains(i), false, "Key was removed but found by 'contains'");
        }
    }

    #[test]
    fn reallocate_test() {
        let mut hashset = MyHashSet::new();

        for i in 0..10000 {
            hashset.add(i);
            assert_eq!(hashset.contains(i), true);
        }
    }

}
