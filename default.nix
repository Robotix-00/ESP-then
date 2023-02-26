{ rustPlatform, ... }:
let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  checkPhase = ''

  '';

  src = ./.;
  cargoSha256 = "sha256-H8iaouyo68fGpfTfrtEH8GcUyZC5HKoKFPq2DEmQlCI=";
}
