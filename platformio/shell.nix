{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    clang
    
    platformio
  ];

  shellHook = ''
    echo -e "\e[1;34mLab "esp32-c" loaded successfully, have fun!\e[0m"
  '';
}
