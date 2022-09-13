extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        static_vcruntime::metabuild();
        embed_resource::compile("icon.rc");
    }
}
