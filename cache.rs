pub struct Cache { map: std::collections::HashMap<String, i32> }
impl Cache {
    pub fn new() -> Self { Self { map: std::collections::HashMap::new() } }
    pub fn set(&mut self, k: &str, v: i32) { self.map.insert(k.to_string(), v); }
}