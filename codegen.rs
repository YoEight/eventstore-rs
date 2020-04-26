#[cfg(not(feature = "es6"))]
pub mod tcp {
    use protoc_rust::Customize;
    use std::path::Path;

    pub fn generate() {
        let generated_file = Path::new("src/internal/messages.rs");

        if !generated_file.exists() {
            protoc_rust::run(protoc_rust::Args {
                out_dir: "src/internal",
                input: &["protos/tcp/messages.proto"],
                includes: &["protos/tcp"],
                customize: Customize {
                    carllerche_bytes_for_bytes: Some(true),
                    carllerche_bytes_for_string: Some(true),
                    ..Default::default()
                },
            })
            .expect("protoc");
        }
    }
}

#[cfg(feature = "es6")]
pub mod es6 {
    pub fn generate() {
        let files = [
            "protos/es6/persistent.proto",
            "protos/es6/streams.proto",
            "protos/es6/shared.proto",
        ];

        tonic_build::configure()
            .build_server(false)
            .out_dir("src/es6/grpc/event_store/client")
            .compile(
                &files,
                &["protos/es6"]
            ).unwrap();
    }
}

#[cfg(feature = "es6")]
use self::es6::generate;
#[cfg(not(feature = "es6"))]
use self::tcp::generate;

fn main() {
    generate();
}
