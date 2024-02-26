{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };


  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, fenix, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        # default = fenix.packages.x86_64-linux.default.toolchain;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
            fenixPkgs = fenix.packages.${system};
            rustcWithWasm = with fenixPkgs; combine [
              default.cargo
              default.rustc
              default.rust-std
              default.clippy
              latest.llvm-tools-preview
              latest.rust-analyzer
              latest.rustfmt
              latest.rust-src
              targets.wasm32-unknown-unknown.latest.rust-std
            ];
            connectedPkgs = pkgs.symlinkJoin {
              name = "balatro-hands-rust-pkgs";
              paths = with pkgs; [
                rustcWithWasm
                wasm-pack
                wasm-bindgen-cli
              ];
            };
          in
          with fenix.packages.${system}; {
            default =
              devenv.lib.mkShell
                {
                  inherit inputs pkgs;
                  modules = [
                    {
                      # https://devenv.sh/reference/options/
                      languages = {
                        rust = {
                          enable = true;
                          channel = "nightly";
                          components = [ ];
                          toolchain = {
                            rust = rustcWithWasm;
                          };
                        };
                      };
                      packages = with pkgs;[
                        connectedPkgs
                      ];
                    }
                  ];
                };
          });
    };
}
