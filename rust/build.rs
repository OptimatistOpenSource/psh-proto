fn main() {
    tonic_build::configure()
        .compile_protos(&["../proto/psh.proto"], &["../proto"])
        .unwrap();
    // tonic_build::compile_protos("../proto/psh.proto").unwrap();
}
