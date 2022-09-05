# Installation

1. install [rustup](https://rustup.rs) and through that the current stable
   version of [Rust](https://www.rust-lang.org)

2. install mrt:

   ```
   cargo install --locked --git github.com/kevgo/mrt`
   ```

3. install the shell wrapper through which you will run mrt:

   ```
   ~/.cargo/bin/mrt activate | source
   ```

Now you can run mrt by calling the shell function `m`. No need to change the
`$PATH` environment variable. This also gives you auto-completion of mrt
arguments. The shell wrapper is necessary for exiting the current shell in
folders that encounter problems.
