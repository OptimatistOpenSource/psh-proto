PSH Protobuf proto project
=========================

This project is shared between different projects that need to communicate with each others via gRPC.

How To Use it?
==============

- Add this project as a submodule in project that you're working on.

```
    git submodule add git@github.com:OptimatistOpenSource/psh-proto.git /path/to/psh-proto
```

- Initialize and Update Submodule:
```
    git submodule update --init --recursive
```

- Reference the protobuf files within it directly from your project. Import the protobuf definitions in your projects as you normally would. 

Rust
====

To automatically use the latest proto files while you're running `cargo build`, add this code snippet in your `build.rs`. For example:

```
se std::process::Command;

fn main() {
    // Update the psh-proto submodule
    let _ = Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status();

    // Additional build steps if needed

    // Cargo will continue with its default build process after this script exits
}
```
