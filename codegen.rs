extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/internal",
        input: &["protos/messages.proto"],
        includes: &["protos"],
    }).expect("protoc");
}
