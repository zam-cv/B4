{ system ? builtins.currentSystem }:

let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "latest";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };

  platformSpecificDeps = if system == "x86_64-linux" then
    [ pkgs.openssl ]
  else if system == "x86_64-darwin" then
    [
      pkgs.libiconv
      pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation 
    ]
  else if system == "x86_64-windows" then
    [ pkgs.mingwW64 ]
  else
    [];

  platformSpecificEnv = if system == "x86_64-linux" then
    {
      OPENSSL_DIR = pkgs.openssl.dev;
    }
  else if system == "x86_64-darwin" then
    {
      ICONV_DIR = pkgs.libiconv;
    }
  else
    {};

in pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.pkg-config
    pkgs.diesel-cli
    pkgs.nodejs_21
  ] ++ platformSpecificDeps;

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}
