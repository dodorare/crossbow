# Crossbundle new command

`crossbundle` uses [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to generate a new project. This means that you need to install it before we proceed.

```sh
cargo install cargo-generate
```

Then you can create a new project:

```sh
crossbundle new project-name
# crossbundle new project-name --template bevy
# crossbundle new project-name --template quad
```

All supported templates you can watch [`here`](https://github.com/dodorare/crossbundle-templates) (each branch = template).
