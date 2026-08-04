pub struct Cache { map: std::collections::HashMap<String, i32> }
impl Cache {
    pub fn new() -> Self { Self { map: std::collections::HashMap::new() } }