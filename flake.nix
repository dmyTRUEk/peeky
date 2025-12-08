{
	description = "Simple image viewer using flake.";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		rust-overlay.url = "github:oxalica/rust-overlay"; # for nightly
	};

	outputs = { self, nixpkgs, rust-overlay }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
			overlays = [ (import rust-overlay) ];
		};
	in {
		devShells.${system}.default = pkgs.mkShell rec {
			packages = with pkgs; [
				libxkbcommon # for minifb
			];
		};

		packages.${system}.default =
		let
			rust = pkgs.rust-bin.nightly.latest.default; # nightly toolchain from the overlay
			cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
			pname = cargoToml.package.name;
			version = cargoToml.package.version;
		in
			pkgs.rustPlatform.buildRustPackage {
				inherit pname version;
				src = self;
				cargoLock.lockFile = ./Cargo.lock;
				nativeBuildInputs = [
					rust
					pkgs.libxkbcommon
				];
			};
		apps.${system}.default = {
			type = "app";
			program = "${self.packages.${system}.default}/bin/${self.packages.${system}.default.pname}";
		};
	};
}
