{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {self, nixpkgs, fenix }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    fe_pkgs = fenix.packages.${system};

    librarys = with pkgs; [
      wayland
      libxkbcommon
      sdl3
      vulkan-loader
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        (fe_pkgs.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ])
        pkg-config
      ] ++ librarys;

      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath librarys}";

      # wgsl-analyzer has wrong name :( this fixes that
      shellHook = ''
        mkdir -p .dev-bin
        ln -sf ${pkgs.wgsl-analyzer}/bin/wgsl-analyzer .dev-bin/wgsl_analyzer
        export PATH="$PWD/.dev-bin:$PATH"
      '';
    };
  };
}
