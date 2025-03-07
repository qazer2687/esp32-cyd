{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    git
    wget
    flex
    bison
    gperf
    python3
    python3Packages.pip
    cmake
    ninja
    ccache
    libffi
    openssl
    dfu-util
    libusb1

    # Cargo
    cargo-generate
    ldproxy
    espup
    espflash
    cargo-espflash

    # Dev Container
    devcontainer
  ];

  nativeBuildInputs = with pkgs; [ rustc rustup cargo gcc rustfmt clippy ];

  # This sets the RUST_SRC_PATH so that Rust tools can find the source
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = ''
    echo -e "\e[1;34mLab "esp32-rust" loaded successfully, have fun!\e[0m"
  '';
}
