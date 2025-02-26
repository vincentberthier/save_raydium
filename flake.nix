{
  description = "Development environment for Save+Raydium";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    solana = {
      url = "github:VincentBerthier/solana";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }@inputs:
    let
      supportedSystems = [
        "i686-linux"
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: (forSystem system f));
      forSystem =
        system: f:
        f rec {
          inherit system;
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };
          lib = pkgs.lib;
          rust-bin = pkgs.rust-bin.nightly.latest.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
            targets = [ "x86_64-unknown-redox" ];
          };
        };
    in
    {
      formatter = forAllSystems ({ pkgs, ... }: pkgs.nixfmt-rfc-style);
      devShells = forAllSystems (
        {
          system,
          pkgs,
          rust-bin,
          ...
        }:
        let
          buildInputs = with pkgs; [
            # Compilation
            rust-bin
            openssl
            pkg-config

            udev
          ];
        in
        {
          default = pkgs.mkShell {
            inherit buildInputs;

            packages = with pkgs; [
              # Compilation
              mold # rust linker

              # Solana from flake
              inputs.solana.packages.${system}.default

              # Utils
              cowsay
              gitmoji-cli # Use gitmojis to commit
              lolcat
              tokei # file lines count

              # Cargo utilities
              cargo-expand # for macro expension
              cargo-nextest # testing framework
              cargo-spellcheck # Spellcheck documentation
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            shellHook = ''
              export PATH="$HOME/.cargo/bin:$PATH"
              echo "Save / Raydium environmont loaded" | cowsay | lolcat
            '';
          };
        }
      );
    };
}
