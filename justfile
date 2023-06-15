dev: build
  cd dev && cargo watch -x run

# === BUILD SCRIPTS === #
build:
  cargo build

build-dev:
  cargo build -p dev

build-v1:
  cargo build -p ver_01
