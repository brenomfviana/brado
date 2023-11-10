# Source: https://github.com/berbiche/sample-flake-rust/blob/master/flake.nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore.nix"; flake = false; };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        gitignoreSrc = pkgs.callPackage inputs.gitignore { };
      in
      rec {

        /*
          https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
        */
        packages.brado =
          let manifest = (pkgs.lib.importTOML ./brado/Cargo.toml).package;
          in pkgs.rustPlatform.buildRustPackage rec {
            pname = "brado";
            version = manifest.version;
            cargoLock.lockFile = ./brado/Cargo.lock;
            src = pkgs.lib.cleanSource ./brado;
            buildPhase = ''
              cargo build --release
            '';
            installPhase = ''
              # ls -alh target/x86_64-unknown-linux-gnu/release/build
              # ls -alh target/x86_64-unknown-linux-gnu/release/deps/brado-7d293e41a114547d
              # file target/x86_64-unknown-linux-gnu/release/deps/brado-7d293e41a114547d

              mkdir -p $out/bin
              cp -v target/*/release/deps/brado-7d293e41a114547d brado

              cp -v brado $out/bin/
            '';

          };

        packages.default = packages.brado;

        apps.brado = {
          type = "app";
          # TODO: o mais correto é definir .meta.mainProgram vs pname?
          program = "${self.packages.${system}.default}/bin/${self.packages.${system}.default.pname}";
        };

        # nix fmt
        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

          buildInputs = with pkgs; [ cargo rustc rustfmt clippy git ];
        };
      });
}
