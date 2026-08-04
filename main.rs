mod cache;
fn main() {
    let mut c = cache::Cache::new();
    c.set("a", 1);
}