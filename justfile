dev: build
  cd dev && cargo watch -x run

# === BUILD SCRIPTS === #
build:
  cargo build

build-dev:
  cargo build -p dev

build-v1:
  cargo build -p ver_01

# === EXPAND SCRIPTS === #
expand:
  cd dev && cargo expand

expand-01:
  cd dev && cargo expand dev_ver_01::v1_types

expand-02:
  cd dev && cargo expand dev_ver_02::v2_types

expand-03:
  cd dev && cargo expand dev_ver_03::v3_types
