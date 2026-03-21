{
  # Pfad zu deiner zentralen Datei anpassen!
  inputs.my-base.url = "path:/Users/dragon/config-files/nix-config/flake-templates";

  outputs = { my-base, ... }: my-base.lib.mkMacShell {
    packages = pkgs: with pkgs; [
      go
      cacert
      git
      gnumake
      libffi
      zlib
      libxml2
    ];
  };
}
