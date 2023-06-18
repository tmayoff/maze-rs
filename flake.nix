{
  description = "";

  inputs = {
	nixpkgs.url = "nixpkgs/nixos-22.11";
	flake-utils.url = "github:numtide/flake-utils";
	naersk.url = "github:nix-community/naersk";
	rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay, ... }:
	flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];

	  pkgs = import nixpkgs { 
		inherit system overlays;
	  };

	  naersk' = pkgs.callPackage naersk {};
	  rustVersion = pkgs.rust-bin.stable.latest.default;

    in rec {
		defaultPackage = naersk'.buildPackage {
			src = ./.;
		};

		devShell = pkgs.mkShell rec {
			nativeBuildInputs = with pkgs; [ 
				pkg-config
				vulkan-tools
			];

			buildInputs = [
				pkgs.alsa-lib
				pkgs.udev
				pkgs.xorg.libX11
             	pkgs.xlibsWrapper
                pkgs.xorg.libXrandr
                pkgs.xorg.libXcursor
                pkgs.xorg.libXi
				pkgs.vulkan-loader

				(rustVersion.override { extensions = [ "rust-src" ]; })
			];

		};
    });
}
