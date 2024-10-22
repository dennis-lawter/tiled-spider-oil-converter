{
  description = "Flake environment for tiled-spider-oil-converter";
  inputs =
  {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs, ... }@inputs:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell
    {
      packages = with pkgs; [
        rustc
        cargo
        
        tiled
      ];
      shellHook = ''
        rustup toolchain install nightly-2024-10-20
        rustup default nightly-2024-10-20
        export RUSTC=$(rustup which rustc)
      '';
    };
  };
}
