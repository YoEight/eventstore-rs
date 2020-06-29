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
        let out_dir = "src/es6/grpc/event_store/client";
        let files = [
            "protos/es6/persistent.proto",
            "protos/es6/streams.proto",
            "protos/es6/shared.proto",
        ];

        tonic_build::configure()
            .build_server(false)
            .out_dir(out_dir)
            .compile(&files, &["protos/es6"])
            .unwrap();

        let gen_dir = std::fs::read_dir(out_dir).unwrap();

        for entry in gen_dir {
            let file = entry.unwrap();
            let filename_string = file.file_name().into_string().unwrap();
            if filename_string.starts_with("event_store.client.") {
                let remaining = filename_string.trim_start_matches("event_store.client.");
                let new_file_name = if remaining == "persistent_subscriptions.rs" {
                    "persistent.rs"
                } else {
                    remaining
                };

                let new_file = file.path().parent().unwrap().join(new_file_name);

                std::fs::rename(file.path(), new_file).unwrap();
            }
        }
    }
}

#[cfg(feature = "es6")]
use self::es6::generate;
#[cfg(not(feature = "es6"))]
use self::tcp::generate;

fn main() {
    generate();
}
