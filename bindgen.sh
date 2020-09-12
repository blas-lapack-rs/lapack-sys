#!/bin/bash
set -eux

# - LAPACK routines in lapack.h has `.*_` naming because `LAPACK_GLOBAL` macro adds last '_'
#   This script filters using this last '_'.
#
# - `char` in C header will convert into `::std::os::raw::c_char`, but we want to keep no_std.
#   This is replaced by sed (FIXME is it possible using bindgen?).
bindgen \
  --whitelist-function="^.*_$" \
  --use-core \
  wrapper.h \
  | sed -e "s/::std::os::raw:://g" \
  > src/lapack.rs
# Re-format due to sed
rustfmt src/lapack.rs
