{
  description = "Tmux status line configured in Rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    naersk.url = "github:nix-community/naersk";
    systems.url = "github:nix-systems/default-linux";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  nixConfig = {
    #extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    #extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs @ {
    flake-parts,
    systems,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.devenv.flakeModule];
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

        devenv.shells.default = {
          name = "muxbar";
          languages.rust.enable = true;
        };

        formatter = pkgs.alejandra;
      };
      flake = {};
    };
}
