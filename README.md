# Getting started with Rust in Linux

Install Rust in Linux:
```
sudo apt update && sudo apt -y install curl
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

To compile:
```
rustc hello-world.rs
```

To run:
```
./hello-world
```

* note: using Rust on Windows can be troublesome for now; [linker issue](https://github.com/rust-lang/rust/issues/30319) ; [Visual Studio issue](https://users.rust-lang.org/t/problems-with-installing-rust-on-windows/9349/2).*

*Use bash in Windows 10 if installed.*