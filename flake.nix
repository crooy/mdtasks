{
  description = "mdtasks - Markdown task manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { pkgs, ... }: {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rust-analyzer
            rustfmt
            clippy

            # Development tools
            git
            direnv
          ];

          shellHook = ''
            echo "📋 Welcome to mdtasks Development Environment!"
            echo "🦀 Rust version: $(rustc --version)"
            echo ""
            echo "📚 Available commands:"
            echo "  cargo run -- list                    - List all tasks"
            echo "  cargo run -- list --status pending   - List pending tasks"
            echo "  cargo run -- list --tag cli         - List tasks with 'cli' tag"
            echo "  cargo run -- show 001               - Show task details"
            echo "  cargo test                           - Run tests"
            echo ""
          '';
        };
      };
    };
}
