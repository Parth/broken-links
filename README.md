# broken-links

`broken-links` will take the output of `git ls-files` search all the files mentioned for anything that looks like a link, and perform a `get` request to see if the link is still valid.

## Build from source:

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```
curl https://sh.rustup.rs -sSf | sh
```

Create binary

```
cargo build
```

This will create a binary and place it in `target/release`. You can add this location to your `$PATH` or copy this binary to a location already in your `$PATH`, or just move it to the repository you want check.
