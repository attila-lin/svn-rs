# svn-rs

A porting of the Subversion (SVN) library and command line tools to Rust.

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
- No early version support
  - BDB support
- All deprecated features and functions are removed

### New features

- More configuration file types
  - TOML, JSON, YAML

## Some Improvements

- Better cross-platform support: Linux, macOS, Windows, and more
- Performance improvements: FIXME:
- Better error handling: FIXME:
- Better Type Definition
- Safety improvements: FIXME:
- Documentation improvements: FIXME:
- Well tested and more unit tests
