fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("src")
        .input("src/protos/MsgIssue.proto")
        .out_dir("src/protos/MsgIssue.rs")
        .run_from_script();
}
