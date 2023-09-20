{
  description = "saffron";
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        naersk' = pkgs.callPackage naersk { };
        nodejs = pkgs.nodejs-19_x;
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nodeDeps = (pkgs.callPackage ./views { }).nodeDependencies;
        tailwind-styles = pkgs.stdenv.mkDerivation {
          name = "tailwind-styles";
          src = ./views;
          buildInputs = [ nodejs ];
          buildPhase = ''
            ln -s ${nodeDeps}/lib/node_modules ./node_modules
            export PATH="${nodeDeps}/bin:$PATH"
            export NODE_ENV=production
            postcss index.tailwind.css -o $out/index.css
          '';
        };
      in
      rec {
        packages = {
          saffron = naersk'.buildPackage {
            pname = "saffron";
            version = "0.1.0";
            root = ./.;
            nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];
            preInstall = ''
              cp ${tailwind-styles}/index.css $src/views/static/index.css
            '';
            meta = {
              description = "A little private file hosting service.";
              homepage = "https://github.com/azuline/saffron";
              license = nixpkgs.lib.licenses.agpl3Plus;
            };
          };
        };
        app = {
          saffron = {
            type = "app";
            program = "${defaultPackage}/bin/saffron";
          };
        };

        defaultPackage = packages.saffron;
        defaultApp = app.saffron;

        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.buildEnv {
              name = "env";
              paths = with pkgs; [
                rust
                nodejs
                node2nix
                nodeDeps
                openssl.dev
                pkg-config
              ];
            })
          ];
          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
            export OPENSSL_DIR="${pkgs.openssl.dev}"
          '';
        };
      }
    );
}
