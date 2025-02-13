fn main() {
    tonic_build::compile_protos("psh.proto").unwrap();
}
