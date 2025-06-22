# Porting documentation

## Do not need `apr` any more

use rust `std` library instead of `apr` for memory management and other utilities.

use third party libraries to replace `apr` functionality, like `regex` for regex operations, `chrono` for date/time operations, etc.

`svn_cl__format_file_size` use `humansize` crate to format file sizes.

## Ignore all `*_dup` functions

just impl `clone` for all types that implement `Clone` trait.

## Ignore all `pool` fields

## Some function do not need return `svn_error_t`

like `svn_opt__split_arg_at_peg_revision`.
