// extern crate protoc_rust_grpc;

// fn main() {
//     // compile protocol buffer using protoc
//     protoc_rust_grpc::Codegen::new()
//         .out_dir("src")
//         .input("jobpb/jobs.proto")
//         .includes(&["jobpb"])
//         .rust_protobuf(true)
//         .run()
//         .expect("error generating protobuf code");
// }
//
//
fn main() {
    tonic_build::configure()
        .out_dir("src/")
        .compile(&["jobpb/jobs.proto"], &["jobpb"])
        .unwrap();
}
