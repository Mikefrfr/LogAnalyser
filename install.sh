#!/bin/bash

echo "[*] Installing LogAnalyser..."
cargo install --path . || {
    echo "[!] Cargo install failed. Make sure Rust is installed."
    exit 1
}

CARGO_BIN="$HOME/.cargo/bin"

if [[ ":$PATH:" != *":$CARGO_BIN:"* ]]; then
    echo "[*] Adding $CARGO_BIN to your PATH..."

    SHELL_NAME=$(basename "$SHELL")

    case "$SHELL_NAME" in
        bash)
            FILE="$HOME/.bashrc"
            ;;
        zsh)
            FILE="$HOME/.zshrc"
            ;;
        fish)
            FILE="$HOME/.config/fish/config.fish"
            ;;
        *)
            FILE="$HOME/.profile"
            ;;
    esac

    if [[ ! -f "$FILE" ]]; then
        touch "$FILE"
        echo "[*] Created $FILE and added PATH export."
    else
        echo "[*] Updated $FILE with PATH export."
    fi

    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$FILE"

    echo "[*] PATH updated. Please restart your terminal or run:"
    echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
else
    echo "[*] $CARGO_BIN is already in your PATH"
fi

echo "[✔] Done! You can now run 'LogAnalyser' from anywhere."
