# svn-rs

A porting of the [Subversion](https://subversion.apache.org/) (SVN) library and command line tools to **Rust**.

**Unfinished**

This project is a work in progress and is not yet functional. It is intended to be a complete rewrite of the Subversion library and command line tools in Rust, with a focus on safety, performance, and maintainability.

## Ambitions

- Port the Subversion library to Rust, with a focus on safety and performance.
- Port the Subversion command line tools to Rust, with a focus on usability and performance.
- Provide a complete and functional Subversion client in Rust.
- Provide a complete and functional Subversion server in Rust.
- Add more features and improvements to the Subversion protocol and library.

## Differences from Subversion

### Do not support any more

- No DVA
- No single thread support anymore
- No early version support
  - BDB support
- All deprecated features and functions are removed
- **ra-serf**: will not be supported currently
- **mod_authz_svn**: will not be supported currently
- **mod_dav_svn**: will not be supported currently
- **libsvn_auth_gonme_keyring**: will never be supported
- **libsvn_auth_kwallet**: will never be supported

### New features

- More configuration file types
  - `TOML`, `JSON`, `YAML` and more
- Async support
  - Tokio
  - io-uring

## Some Improvements

- Better cross-platform support: Linux, macOS, Windows, WASM and more
- Less code, less complexity, less bugs
- Performance improvements: SIMD, asynchronous I/O, and more
- Better error handling
- Better Type Definition
- Safety improvements: FIXME:
- Documentation improvements: FIXME:
- Well tested and more unit tests
