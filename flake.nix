{
  description = "A flake for building the ESP-THEN libary for receiving espnow packets on PC";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

      in rec {
        packages = {
        };

        devShells = rec {
          default = rust;

          rust = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
            ];
          };

          hardware = pkgs.mkShell {
            buildInputs = with pkgs; [
              platformio
              avrdude
            ];
          };
        };
      }
    );
}
