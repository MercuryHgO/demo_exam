{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { 
          inherit system;
        };
        naersk-lib = pkgs.callPackage naersk { };
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
        ];

      in
      {
        # defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            rust-analyzer
            pkg-config
            wayland
            xorg.libX11
            libxkbcommon
            gcc
          ];

          
          PKG_CONFIG_PATH="${builtins.concatStringsSep ":" (builtins.map (pkg: "${pkg.dev}/lib/pkgconfig") pkgConfigDeps)}";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
          LD_LIBRARY_PATH="/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU wayland libxkbcommon])}";
          DATABASE_URL="sqlite://${./interface/data.sqlite}";
          RUST_SRC_PATH="${rustPlatform.rustLibSrc}";

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
