# RUST INSTALLATION AND RUN FILES IN FEDORA
Rust resource: https://doc.rust-lang.org/book/print.html

## installation
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
```
sudo dnf install rust cargo
```
it'll install rustc and cargo
NOTE: rustc is the compiler and cargo is the package manager



### check version
```
rustc --version
```
```
cargo --version
```
### Process of creating rust file and run
#### create
```
touch main.rs
```
#### compile
```
rustc main.rs
```
#### run
```
./main
```


### Building and running cargo project

#### create cargo project
`cargo new hello_cargo`
#### enter to the project_dir
`cd hello_cargo/`
`vi Cargo.toml`
`vi src/main.rs`

#### check before build and run
`cargo check`

#### build the project and run
`cargo build`
or
`cargo build --release`
`./target/debug/hello_cargo` 

#### or directly build and run
`cargo run`
