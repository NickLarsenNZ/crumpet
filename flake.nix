{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };
    outputs = {self, nixpkgs, flake-utils}:
        flake-utils.lib.eachDefaultSystem (system:
            let
                name = "simple";
                src = ./.;
                pkgs = nixpkgs.legacyPackages.${system};
            in {
                packages.default = derivation {
                    inherit name src system;
                    builder = with pkgs; "${bash}/bin/bash";
                    args = ["-c" "echo foo > $out"];
                };
            }
        );
}
