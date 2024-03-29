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
        pkgs = import nixpkgs {
          inherit system;
          config.allowBroken = true;
          config.allowUnsupportedSystem = true;
          overlays = [ (import rust-overlay) ];
        };

      in rec {
        packages.default = pkgs.callPackage ./. {};
        checks.default = packages.default;

        devShells = rec {
          default = rust;

          rust = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rust-analyzer
              clippy
              rustfmt
              fzf
            ];

            shellHook = ''
              interface=$(
                {
                  echo "none"
                  (iw dev | awk '$1=="Interface"{print $2}')
                } | fzf)

              if [ "$interface" != "none" ]; then
                sudo ip link set $interface down
                sudo iw $interface set monitor none
                sudo ip link set $interface up
                echo "$interface is now in monitor mode";
                export interface=$interface;
              else
                echo "no interface put into monitor mode";
              fi;
              '';
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
