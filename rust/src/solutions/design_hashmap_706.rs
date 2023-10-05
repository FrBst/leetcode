pub struct MyHashMap {
    map: Vec<Vec<(i32,i32)>>
}

const BUCKETS: i32 = 1000;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            map: vec![Vec::new(); BUCKETS as usize]
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {

        let bucket = self.get_bucket(key);

        for pair in &mut self.map[bucket] {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }

        self.map[bucket].push((key,value));
    }

    fn get(&self, key: i32) -> i32 {
        let bucket = self.get_bucket(key);

        for pair in &self.map[bucket] {
            if pair.0 == key {
                return pair.1;
            }
        }

        return -1;
    }
    
    fn remove(&mut self, key: i32) {
        let bucket = self.get_bucket(key);

        let mut found_index = None;
        let bucket_len = self.map[bucket].len();
        for i in 0..bucket_len {
            if self.map[bucket][i].0 == key {
                found_index = Some(i);
                break;
            }
        }

        match found_index {
            None => (),
            Some(index) => {
                self.map[bucket][index] = self.map[bucket][bucket_len - 1];
                self.map[bucket].remove(bucket_len - 1);
            }
        }
    }

    fn get_bucket(&self, key: i32) -> usize {
        ((31 * key + 7) % BUCKETS) as usize
    }
}

// /**
//  * Your MyHashMap object will be instantiated and called as such:
//  * let obj = MyHashMap::new();
//  * obj.put(key, value);
//  * let ret_2: i32 = obj.get(key);
//  * obj.remove(key);
//  */