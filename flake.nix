{
  description = '':
    Gomoku "l'eau ça rouille" Flake
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    nixpkgs-cargo-1-75.url = "github:NixOS/nixpkgs/9a9dae8f6319600fa9aebde37f340975cab4b8c0";
  };

  outputs = { self, nixpkgs, nixpkgs-cargo-1-75, ... }:
    let
      system = "x86_64-linux";
    in {
      devShells."${system}".default = let
        pkgs = import nixpkgs {
          inherit system;
        };
        cargo = (import nixpkgs-cargo-1-75 { inherit system; }).cargo;
      in pkgs.mkShell {
        packages = with pkgs; [
          bashInteractiveFHS
          gnumake

          cargo
        ];

        # Set an environment variable that will be read by the Makefile
        BUILD_MODE="DEBUG";

        shellHook = ''
          echo Custom Cargo nix-shell !
        '';
      };
    };
}
