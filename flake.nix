{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    oxalica-rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, oxalica-rust }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend oxalica-rust.overlays.default;
        rustStable = pkgs.rust-bin.stable.latest;
        rustNightly = pkgs.rust-bin.nightly.latest;
        rust = rustStable.minimal;

        unpack = drv: pkgs.runCommand "${drv.name}-unpacked" {} ''
          mkdir extracted
          cd extracted
          unpackFile ${drv}
          cp -a ./* $out
        '';

        ffmpeg = pkgs.ffmpeg_6;
      in {
        devShell = with pkgs; mkShell {
          buildInputs = [
            clang pkg-config
            nasm

            (enableDebugging ffmpeg).out
            (enableDebugging ffmpeg).dev

            (rust.override {
              extensions = [
                "rust-src"
                "clippy"
              ];
            })

            # necessary for unstable format options
            rustNightly.rustfmt

            (writeShellScriptBin "gdb" ''
              RUSTC_COMMIT="$(rustc -vV | awk '/commit-hash: (.+)/ { print $2 }')"

              exec ${gdb}/bin/gdb \
                -ex "set substitute-path '/rustc/$RUSTC_COMMIT' '${rustStable.rust-src}/lib/rustlib/src/rust'" \
                -d ${unpack ffmpeg.src} \
                "$@"
            '')
          ];

          RUST_BACKTRACE = 1;
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        };
      });
}
