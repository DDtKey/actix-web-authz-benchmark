#!/bin/sh
#set -e

HOST="http://localhost:8081"
ALLOWED_ENDPOINT="${HOST}/allowed"
DENIED_ENDPOINT="${HOST}/denied"

ACTIX_WEB_GRANTS="actix-web-grants"
CASBIN_RS="casbin-rs"

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

main() {
  require_cmd cargo
  require_cmd wrk

  cargo build --release # build release packages for benchmarking
  start_bench "${ACTIX_WEB_GRANTS}"
  start_bench "${CASBIN_RS}"

  return 0;
}

start_bench() {
  printf "${RED}Benchmark: ${GREEN}[%s]${NC} \n" "$1"
  cargo run --bin "$1-example" --release &
  APP_PID=$!
  sleep 3 # wait to start the app

  start_wrk "${ALLOWED_ENDPOINT}"
  start_wrk "${DENIED_ENDPOINT}"

  kill "${APP_PID}"
}

start_wrk() {
  wrk -t4 -c400 -d30s "$1"
}

require_cmd() {
    if ! cmd_exists "$1"; then
        err "'$1' is required (command not found)."
    fi
}

cmd_exists() {
    command -v "$1" > /dev/null 2>&1
}

err() {
    "$1" >&2
    exit 1
}

main || exit 1
