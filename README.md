# tokyocabinet-rust
使用[rust-bindgen](https://github.com/rust-lang/rust-bindgen)
封装系统环境中的tokyobinetdb。
因此需要在系统中安装c实现的tokyocabinet。

test代码参考[tokyocabinet-sys](https://github.com/ehiggs/tokyocabinet-sys)
### Todo
How to use c function in Rust safe mode?

### warning!
1. libtokyocabinet use `u128`, make rust compiler warning。
2. call C function seems all unsafe.

### build && test
build
```shell
cargo build
```
test, and println
```shell
cargo test -- --nocapture
```

### support
- [x] Hash Table DB(`tchdb`)