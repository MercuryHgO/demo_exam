{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { 
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustVersion = "latest";
        rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
          extensions = [
            "rust-src" # for rust-analyzer
            "rust-analyzer"
          ];
          targets = [
            "x86_64-pc-windows-gnu"
            "x86_64-pc-windows-gnullvm"
          ];
        };
        # naersk-lib = pkgs.callPackage naersk { };
        pkgConfigDeps = with pkgs; [
          gtk3
          glib
          cairo
          pango
          harfbuzz
          atk
          gdk-pixbuf
          webkitgtk_4_1
          librsvg
          libsoup_3
          openssl
          sqlite
        ];

      in
      {
        # defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; with builtins; mkShell {
          buildInputs = [
            rust
            rustfmt
            pre-commit
            rustPackages.clippy
            rust-analyzer
            pkg-config
            wayland
            xorg.libX11
            libxkbcommon
            gcc
            sqlite
            sqlx-cli
            pkgsCross.mingwW64.stdenv.cc
          ];

          
          PKG_CONFIG_PATH="${concatStringsSep ":" (map (pkg: "${pkg.dev}/lib/pkgconfig") pkgConfigDeps)}";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${pkgsCross.mingwW64.windows.pthreads}/lib";
          LD_LIBRARY_PATH="/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU wayland libxkbcommon sqlite])}";
          DATABASE_URL="sqlite:///home/bittermann/projects/module2/interface/data.sqlite";
          # RUST_SRC_PATH="${rustPlatform.rustLibSrc}";
          # ZHOPA="ZHOPA";

          shellHook = ''

            tmux new-session -d -t module2-shell

            tmux split-window -h -t module2-shell
            tmux resize-pane -t module2-shell:0.1 -x 20%

            tmux send-keys -t module2-shell:0.0 'hx' C-m

            # Docker-compose window
            # tmux new-window -t module2-shell

            # tmux split-window -h -t module2-shell:1

            # tmux send-keys -t module2-shell:1.0 'docker-compose up -d' C-m

            # tmux send-keys -t module2-shell:1.1 '
            #   while true; do
            #     docker-compose attach database || { echo "Container has stopped. Reattaching..."; sleep 2; };
            #   done ' C-m

            tmux attach-session -t module2-shell

            while tmux has-session -t module2-shell; do sleep 1; done
            exit
          '';
        };
      }
    );
}
