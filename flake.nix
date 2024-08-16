{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          runtimeDeps = with pkgs; [
          ];
          buildDeps = with pkgs; [
            clang
            lld
            lldb
            pkg-config
            rustPlatform.bindgenHook
          ];
          devDeps = with pkgs; [
            cargo-deny
            cargo-edit
            cargo-msrv
            cargo-nextest
            cargo-watch
            # (cargo-whatfeatures.overrideAttrs (oldAttrs:
            #   {
            #     # version = "0.9.11";
            #     cargoBuildFlags = [
            #       "--no-default-features"
            #       "--features=rustls"
            #     ];
            #   }))
            clang
            just
            gdb
            lld
            lldb
            nushell
            (surrealdb.overrideAttrs (oldAttrs: rec
            {
              pname = "surrealdb";
              version = "1.5.4";
              src = fetchFromGitHub {
                owner = "surrealdb";
                repo = "surrealdb";
                rev = "6e9d04d4d7801c359cf3b5ca4398d07efd99bb6a";
                sha256 = "sha256-PeCb76H0YNpTaTOUOKvs8WB3EHN1zRS+AFoWtlYPMSM=";
              };
              cargoDeps = oldAttrs.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-1VSn9twqTysGdx/NmAQkF1u7fQkrhTJk8sfd4rWg9tw=";
              });
            }
            ))
            panamax
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              # Uncomment if your cargo tests require networking or otherwise
              # don't play nicely with the nix build sandbox:
              # doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              shellHook = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              '';
              LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs
            {
              inherit system;
              overlays = [ (import inputs.rust-overlay) ];
              config.allowUnfreePredicate = pkg: builtins.elem (lib.getName pkg) [
                "surrealdb"
              ];
            };

          packages.default = self'.packages.base;
          devShells.default = self'.devShells.stable;

          packages.base = (rustPackage "");
          packages.bunyan = (rustPackage "bunyan");
          packages.tokio-console = (rustPackage "tokio-console");

          devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })));
          devShells.stable = (mkDevShell (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          }));
          devShells.msrv = (mkDevShell (pkgs.rust-bin.stable.${msrv}.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          }));
        };
    };
}
