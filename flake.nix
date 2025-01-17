{
  description = "Tmux status line configured in Rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    naersk.url = "github:nix-community/naersk";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs = inputs @ {
    flake-parts,
    systems,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import systems;

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.
        packages = rec {
          default = muxbar;
          muxbar = inputs.naersk.lib.${system}.buildPackage {
            src = ./.;
            meta = {
              mainProgram = "muxbar";
              description = "Tmux status line configured in Rust";
              homepage = "https://github.com/Dlurak/muxbar";
              license = inputs.nixpkgs.lib.licenses.gpl3Only;
            };
          };
        };

        devShells.default = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        formatter = pkgs.alejandra;
      };
      flake = {};
    };
}
