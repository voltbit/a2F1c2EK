extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["jobpb/jobs.proto"])
        .include("jobpb")
        .run()
        .expect("protoc");
}
