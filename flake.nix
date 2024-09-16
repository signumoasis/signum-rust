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
            cargo-expand
            cargo-msrv
            cargo-nextest
            cargo-watch
            (cargo-whatfeatures.overrideAttrs (oldAttrs: rec
            {
              pname = "cargo-whatfeatures";
              version = "0.9.13";
              src = fetchFromGitHub {
                owner = "museun";
                repo = "cargo-whatfeatures";
                rev = "v0.9.13";
                sha256 = "sha256-YJ08oBTn9OwovnTOuuc1OuVsQp+/TPO3vcY4ybJ26Ms=";
              };
              cargoDeps = oldAttrs.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-8pccXL+Ud3ufYcl2snoSxIfGM1tUR53GUrIp397Rh3o=";
              });
              cargoBuildFlags = [
                "--no-default-features"
                "--features=rustls"
              ];
            }))
            clang
            just
            gdb
            lld
            lldb
            nushell
            (surrealdb.overrideAttrs (oldAttrs: rec
            {
              pname = "surrealdb";
              version = "2.0.0";
              src = fetchFromGitHub {
                owner = "surrealdb";
                repo = "surrealdb";
                rev = "v2.0.0-alpha.10";
                sha256 = "sha256-PYZHPQ/PqdaWEvhp5Iu0O8FmyWCZlB2TlCyI9ofWHzQ=";
              };
              cargoDeps = oldAttrs.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-L14D5Cf5kDpfNiRS28a8uprTiuIsys5srgR7Ah0wgic=";
              });
              cargoBuildFlags = [
              ];
            }
            ))
            panamax
            sass
            tailwindcss
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
                exec zellij --layout ./zellij_layout.kdl
              '';
              LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
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
