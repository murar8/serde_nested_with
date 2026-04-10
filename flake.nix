{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.callPackage (
        {
          mkShell,
          cargo,
          rustc,
          rustfmt,
          clippy,
          rust-analyzer,
          rustPlatform,
          pre-commit,
          taplo,
          nodePackages,
        }:
        mkShell {
          strictDeps = true;
          nativeBuildInputs = [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
            pre-commit
            taplo
            nodePackages.prettier
          ];
          RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
        }
      ) { };
    };
}
