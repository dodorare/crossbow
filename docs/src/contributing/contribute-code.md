# How to Contribute Code

Would you like to contribute code to Crossbow? Here's how!

1. Fork the [dodorare/crossbow](https://github.com/dodorare/crossbow) repository on GitHub, you'll need to create a GitHub account if you don't have one already.*
2. Make your changes in a local clone of your fork.
3. For a higher chance of CI passing the first time, consider run these commands from the root of your local clone:
    1. `cargo fmt --all -- --check` (remove --check to let the command fix found problems)
    2. `cargo clippy --all-targets --all-features -- -D warnings -A clippy::unnecessary-unwrap -A clippy::too-many-arguments`
    3. `cargo test --all-targets --workspace`
4. Push your changes to your fork and open a Pull Request.
5. Respond to any CI failures or review feedback.
<!-- 6. Remember to follow Crossbow's Code of Conduct, and thanks for contributing! -->

> The same steps apply for any other repository in the organization that you would like to contribute to.
