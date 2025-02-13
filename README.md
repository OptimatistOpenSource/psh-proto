# PSH Protobuf Project

This project is shared between different projects that need to communicate with each others via gRPC.

# How To Use it?

## Manual

- Add this project as a submodule in project that you're working on.

```
    git submodule add git@github.com:OptimatistOpenSource/psh-proto.git /path/to/psh-proto
```

- Initialize and Update Submodule:

```
    git submodule update --init --recursive
```

- Reference the protobuf files within it directly from your project. Import the protobuf definitions in your projects as you normally would.

## Rust

Add this code snippet in your `Cargo.toml`

```toml
psh-proto = { git = "ssh://git@github.com/OptimatistOpenSource/psh-proto.git", rev = "..." }
```
