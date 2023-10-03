//RUST INSTALLATION AND RUN FILES IN FEDORA
//https://doc.rust-lang.org/book/print.html

//installation
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
sudo dnf install rust cargo
//it'll install rustc and cargo || rustc is the compiler and cargo is the package manager



//version
rustc --version

//create file main.rs
//compile
rustc main.rs
//run
./main




-------------------------------------------------

//Building and running cargo project
//check version
cargo --version
rustc --version

mkdir hello_world_project_with_cargo
cd hello_world_project_with_cargo/
cargo new hello_cargo //create cargo project
cd hello_cargo/      //enter to the project_dir
ls -al   //creates cargo.toml,.git
vi Cargo.toml 
vi src/main.rs 

//check before build and run
cargo check

//build the project and run
cargo build    (or cargo build --release)
./target/debug/hello_cargo 

//or directly build and run
cargo run


------------------------------------------------

currenly studying: https://doc.rust-lang.org/book/print.html#references-and-borrowing
