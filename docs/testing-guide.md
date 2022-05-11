# 📝 Testing guide

## 🏃‍♀️ How to run tests in crossbow

In `crossbow`, we have unit tests, integration tests, and examples. To run tests, you will need to set up an android or apple environment (you can find more information in [README](./README.md)).

If you want to run tests for our `crossbundle` crate, you can make it by the following steps: download this repository, proceed to the `crossbundle/tools` directory, and run `cargo test`. It will run all tests for the `crossbundle-tools` crate that is primarily used by `crossbundle`.

To test the `crossbundle` crate, we recommend installing it first and then building applications on your own (we have a tutorial on how to get started [here](./main-hello-world.md)).

## ❗ Test cli wrappers

If you want to test underlying libraries, you can clone one of our wrapper libraries and run tests there (like [android-tools-rs](https://github.com/dodorare/android-tools-rs)).

## ❗ In case of issues

Feel free to open github issues or pull requests - we will be happy to fix/review them.
