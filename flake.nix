{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
  };

  description = "Concurrently outputs lines from multiple files";

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      flake = {
        homeManagerModules.default = ./home-module.nix;
      };
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        self',
        config,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };

        overlayAttrs = {
          inherit (config.packages) asyncat;
        };

        packages.asyncat = pkgs.rustPlatform.buildRustPackage {
          pname = "asyncat";
          version = "0.1.0";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with nixpkgs.lib; {
            description = "Concurrently outputs lines from multiple files";
            homepage = "https://github.com/nilp0inter/asyncat";
            license = licenses.gpl3;
            platforms = platforms.linux;
            maintainers = with maintainers; [nilp0inter];
            mainProgram = "asyncat";
          };
        };

        packages.default = self'.packages.asyncat;

        formatter = pkgs.alejandra;
      };
    };
}
