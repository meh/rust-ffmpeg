let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in

with (import <nixpkgs> {
  overlays = [mozilla];
});

mkShell {
  name = "ffmpeg-sys";

  buildInputs = [
    # For building.
    clang rustChannels.stable.rust pkg-config ffmpeg
  ];

  RUST_BACKTRACE = 1;
  RUSTFLAGS = "-C target-cpu=native";

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
}
