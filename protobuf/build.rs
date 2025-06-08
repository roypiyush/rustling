fn main() {
    prost_build::compile_protos(&["src/proto/hello.proto"], &["src/proto"]).unwrap();
}
