{
  description = "Crumpet workspace";

  # tip: use `nix flake metadata` to see which inputs depend on existing inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    # rust-toolchain
    # TODO (@NickLarsenNZ): Look into https://github.com/nix-community/fenix. Is it only nightly?
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Build cargo projects
    # NOTE (@NickLarsenNZ): Examples here https://github.com/ipetkov/crane/blob/master/examples/quick-start-workspace/flake.nix
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # for cargo-audit, cargo-deny, etc
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, advisory-db }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Setup the rust-toolchain
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml; # runtime inputs
        rustToolchainExtensions = (rustToolchain.override {
          extensions = [ "rust-src" ];
        });
        RUST_SRC_PATH = "${rustToolchainExtensions}/lib/rustlib/src/rust/library";

        # Cargo via crane
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;

        # Create a filter for cargo sources and test fixtures (to be used to filter src in tests)
        fixturesFilter = path: _type: builtins.match "^\./fixtures" path != null;
        fixturesOrCargo = path: type:
          (fixturesFilter path type) || (craneLib.filterCargoSources path type);

        # Common args used for building deps and workspace member crates
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
          # Needs a pname here, otherwise you need to provide it in the workspace Cargo.toml:
          # ```toml
          # [workspace.metadata.crane]
          # name = "crumpet-workspace"
          # ```
          pname = "crumpet-workspace";
          strictDeps = true;
        };

        # TODO (@NickLarsenNZ): Look into hakari and cachix
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Extend commonArgs with crate name and version
        individualCrateArgs = crate: commonArgs // {
          inherit (craneLib.crateNameFromCargoToml { cargoToml = "${src}/crates/${crate}/Cargo.toml"; }) pname version;

          # TODO (@NickLarsenNZ): Look into cargo-nextest, add it into checks:
          # See: https://github.com/ipetkov/crane/blob/8a68b987c476a33e90f203f0927614a75c3f47ea/examples/quick-start-workspace/flake.nix#L133-L140
          doCheck = false;
        };

        # Helper function for building a crate by name
        cargoBuildForCrate = crate: craneLib.buildPackage (individualCrateArgs crate // {
          inherit src;
          cargoExtraArgs = "-p ${crate}";
        });

        # The crates to build. Be sure to inherit them in the checks
        crumpet-cli = cargoBuildForCrate "crumpet-cli";

        nativeBuildInputs = with pkgs; [ rustToolchain rustToolchainExtensions ]; # compile time inputs
        buildInputs = with pkgs; [ ]; # runtime inputs
      in
      with pkgs; {
        # nix flake check
        checks = {
          inherit crumpet-cli;

          cargo-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            # TODO (@NickLarsenNZ): Deny clippy warnings
            # cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            cargoClippyExtraArgs = "--all-targets";
          });

          cargo-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          cargo-fmt = craneLib.cargoFmt (commonArgs // {
            inherit cargoArtifacts;
          });

          cargo-audit = craneLib.cargoAudit (commonArgs // {
            inherit cargoArtifacts advisory-db;
          });

          # TODO (@NickLarsenNZ): Make a deny.toml
          # cargo-deny = craneLib.cargoDeny (commonArgs // {
          #   inherit cargoArtifacts;
          # });

          # use cargo-nextest instead of cargo test. set doCheck = false
          cargo-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            src = lib.cleanSourceWith {
              src = ./.; # The original, unfiltered source
              filter = fixturesOrCargo;
              name = "source"; # Be reproducible, regardless of the directory name
            };
            partitions = 1;
            partitionType = "count";
            # withLlvmCov = true; # partitions must be 1
            cargoNextestExtraArgs = "--no-capture";
          });

        };

        # nix build
        # nix build .#<name>
        packages = {
          inherit crumpet-cli;
          default = crumpet-cli;
        };

        # nix run
        apps.default = flake-utils.lib.mkApp {
          drv = crumpet-cli;
        };

        # nix develop
        # nix develop .#<name>
        devShells.default = mkShell {
          inherit RUST_SRC_PATH;
          inputsFrom = [ crumpet-cli ];
        };
      }
    );
}
