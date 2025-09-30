{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/a7abebc31a8f60011277437e000eebcc01702b9f";
    rust-overlay.url = "github:oxalica/rust-overlay/02227ca8c229c968dbb5de95584cfb12b4313104";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        host = pkgs.stdenv.hostPlatform.config; # e.g., aarch64-apple-darwin

        toolchain = p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rustfmt" "clippy" ];
          targets = [ host "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain(toolchain);
        frameworks = pkgs.darwin.apple_sdk.frameworks;

        # An LLVM build environment
        dependencies = with pkgs; [
          trunk
          wasm-bindgen-cli
          dioxus-cli
          protobuf
          grpcurl
          grpcui
          yarn
          nodejs
          ltex-ls-plus
          lychee
          chafa
          perl
          llvmPackages.bintools
          openssl
          openssl.dev
          libiconv 
          pkg-config
          libclang.lib
          libz
          clang
          pkg-config
          rustPlatform.bindgenHook
          lld
          coreutils
          gcc
          rust
          dafny
          python311
           (texlive.withPackages (ps: with ps; [
            chktex
            collection-latex
            collection-latexrecommended
            collection-mathscience
            collection-plaingeneric
            collection-bibtexextra
            collection-langother
            collection-pictures
            collection-pstricks
            collection-metapost
            collection-xetex
            collection-luatex
            collection-context
            collection-formatsextra
            collection-publishers
            collection-texworks
            # Additional packages for section markers and document structure
            amsmath
            geometry
            hyperref
            xcolor
            enumitem
            mdframed
            pgf
            pgfplots
            float
            caption
            babel
            microtype
            fontspec
            unicode-math
            polyglossia
            biblatex
            csquotes
            cleveref
            etoolbox
            xifthen
            xstring
            # Special symbols
            bbding
            marvosym
            wasysym
            # Text formatting
            cancel
            ulem
            soul
            xcolor
          ]))
        ] ++ lib.optionals stdenv.isDarwin [
          frameworks.Security
          frameworks.CoreServices
          frameworks.SystemConfiguration
          frameworks.AppKit
          libelf
        ] ++ lib.optionals stdenv.isLinux [
          udev
          systemd
          bzip2
          elfutils
          jemalloc
          alsa-lib
        ];

        # Specific version of toolchain
        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          targets = [ host "wasm32-unknown-unknown" ];
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
    
      in {
        devShells = rec {
          default = docker-build;
          docker-build = pkgs.mkShell {
            ROCKSDB = pkgs.rocksdb;
            OPENSSL_DEV = pkgs.openssl.dev;

            hardeningDisable = ["fortify"];

            buildInputs = with pkgs; [
              # rust toolchain
              (toolchain pkgs)
            ] ++ dependencies;

            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib/";

            shellHook = ''
              #!/usr/bin/env ${pkgs.bash}

              set -e

              # Export linker flags if on Darwin (macOS)
              if [[ "${pkgs.stdenv.hostPlatform.system}" =~ "darwin" ]]; then
                export MACOSX_DEPLOYMENT_TARGET=$(sw_vers -productVersion)
                export LDFLAGS="-L/opt/homebrew/opt/zlib/lib"
                export CPPFLAGS="-I/opt/homebrew/opt/zlib/include"
              fi

              # Add ./target/debug/* to PATH
              export PATH="$PATH:$(pwd)/target/debug"

              # Add ./target/release/* to PATH
              export PATH="$PATH:$(pwd)/target/release"

              # Copy over ./githooks/pre-commit to .git/hooks/pre-commit
              cp $(pwd)/.githooks/pre-commit $(pwd)/.git/hooks/pre-commit
              chmod +x $(pwd)/.git/hooks/pre-commit

              # chafa --size 30x30 --animate false --colors 8 --center true ./assets/ramate-transparent.png

              echo ""
              echo "Roadline"
              echo "Create roadmaps from Markdown."
            '';
          };
        };
      }
    );
}
