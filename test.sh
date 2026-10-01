#! /bin/bash
set -o errexit -o nounset -o pipefail
cd "$(dirname "$0")"

run() {
  echo 'exec> cargo' "$@" >&2
  cargo "$@"
}

cfg() {
  echo "CONFIGURATION: ${*:-'(none)'}" >&2
  run clippy --locked "$@" -- -D warnings
  run build --locked "$@"
  run test --locked "$@"
  RUSTDOCFLAGS='-D warnings' run doc --locked --no-deps "$@"
}

run fmt -- --check

cfg --no-default-features
cfg --all-features
cfg
cfg --features tokio_process
cfg --features async_process
