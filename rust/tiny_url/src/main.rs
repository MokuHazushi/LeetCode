// https://leetcode.com/problems/encode-and-decode-tinyurl/

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

struct Codec {
    	urls: HashMap<u64, String>,
}

impl Codec {
    fn new() -> Self {
        Codec {
            urls: HashMap::new(),
        }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(long_url.as_bytes());
        let hash = hasher.finish();

        self.urls.insert(hash, String::from(&long_url));

        let mut tiny_url = String::from("https://tinyurl.com/");
        tiny_url.push_str(&hash.to_string());
        tiny_url
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let index = short_url.rfind('/').unwrap();
        let hash = short_url.as_str()[index+1..].parse::<u64>().unwrap();

        self.urls.get(&hash).unwrap().to_string()
    }
}


fn main() {
    println!("Encore and decode URLs!");
    let mut codec = Codec::new();
    let test_set = vec![
        "https://google.com",
        "https://leetcode.com/problems/design-tinyurl"
        ];

    for test in test_set {
        let encoded_url = codec.encode(test.to_string());
        let decoded_url = codec.decode(encoded_url.to_string());
        println!("URL = {}", test);
        println!("\tEncode URL= {}", encoded_url);
        println!("\tDecode URL= {}", decoded_url);
    }
}
