fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["../proto/psh.proto"], &["../proto"])
        .unwrap();
    // tonic_build::compile_protos("../proto/psh.proto").unwrap();
}
