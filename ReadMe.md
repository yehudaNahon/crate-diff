hi 
this is a simple utility i wrote to help sync crates.io into a remote no-prem storage pc. 
i use panamax to download all the rustup and crates file and copy only the changed crates
to download crates.io and rustup run
```sh
cargo install panamax
panamax init mirror
panamax sync mirror
```  
then when you want to copy only the changed crates run
```sh
cargo run -- mirror/crates.io-index mirror/crates packed-crates {commit-to-start-with}
```