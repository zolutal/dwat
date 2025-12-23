{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        # non-rust run time dependencies.
        buildInputs = with pkgs; [];

        # non-rust build time dependencies.
        nativeBuildInputs = with pkgs; [];
      in
      rec {
        defaultPackage = packages.app;
        packages =
          {
            app = naersk'.buildPackage {
              # Naersk will look for a `Cargo.toml` in this directory
              src = ./.;
              nativeBuildInputs = nativeBuildInputs;
              buildInputs = buildInputs;
            };
            container = pkgs.dockerTools.buildImage
              {
                name = "app";
                config = {
                  entrypoint = [ "${packages.app}/bin/app" ];
                };
              };
          };

        devShell = pkgs.mkShell {
          # tools for our dev-shell but that aren't required to build.
          nativeBuildInputs = with pkgs;
            [
              nixpkgs-fmt
              cmake
              rustc
              cargo
              clippy
            ] ++ buildInputs ++ nativeBuildInputs;
        };
      }
    );
}
