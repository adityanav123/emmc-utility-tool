#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="${1:-.}"

echo "[1/7] Checking project dir: ${PROJECT_DIR}"
cd "$PROJECT_DIR"


have_cmd() { command -v "$1" >/dev/null 2>&1; }

as_root() {
  if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
    sudo "$@"
  else
    "$@"
  fi
}

echo "[2/7] Installing system dependencies (Docker, curl, build basics)..."
as_root dnf -y install curl ca-certificates git docker

# Enable and start Docker
echo "[3/7] Enabling + starting Docker..."
as_root systemctl enable --now docker

if ! getent group docker >/dev/null; then
  echo "[info] 'docker' group not found; creating..."
  as_root groupadd docker || true
fi

if ! id -nG "$USER" | tr ' ' '\n' | grep -qx docker; then
  echo "[info] Adding user '$USER' to docker group (you may need to log out/in once)."
  as_root usermod -aG docker "$USER"
fi

DOCKER="docker"
if ! docker ps >/dev/null 2>&1; then
  if as_root docker ps >/dev/null 2>&1; then
    DOCKER="sudo docker"
    echo "[warn] Docker needs sudo in this session. After logout/login, it should work without sudo."
  else
    echo "[error] Docker daemon doesn't seem to be running."
    echo "        Try: sudo systemctl status docker"
    exit 1
  fi
fi

echo "[4/7] Installing Rust toolchain via rustup (if needed)..."
if ! have_cmd rustup; then
  curl -fsSL https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
else
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env" || true
fi

rustup update

echo "[5/7] Installing cross (if needed)..."
if ! have_cmd cross; then
  cargo install cross
fi

echo "[6/7] Adding common cross targets..."
rustup target add \
  x86_64-unknown-linux-musl \
  aarch64-unknown-linux-musl \
  armv7-unknown-linux-musleabihf || true

echo "[7/7] Building release binaries with cross..."
TARGETS=(
  "x86_64-unknown-linux-musl"
  "aarch64-unknown-linux-musl"
  "armv7-unknown-linux-musleabihf"
)

for t in "${TARGETS[@]}"; do
  echo "==> cross build --release --target ${t}"
  cross build --release --target "$t"
done

echo
echo "Done."
echo "Artifacts:"
for t in "${TARGETS[@]}"; do
  echo "  target/${t}/release/"
done
echo
echo "Note: If Docker required sudo, log out/in (or reboot) so your docker group membership applies."
