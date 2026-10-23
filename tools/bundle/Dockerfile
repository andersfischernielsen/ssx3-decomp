# Cross-build image for tools/bundle.py: Linux (x86_64, aarch64) and Windows
# (x86_64, MinGW) builds of the ssx3 window binary from any host with Docker.
# The repository (with the generated code built from your game) is mounted
# at run time; nothing of the game goes into the image.
FROM rust:1-bookworm

# The C toolchain for the other Linux architecture (the native one comes
# with the image), the sound/gamepad libraries for both, and MinGW.
RUN set -eux; native=$(dpkg --print-architecture); \
    case "$native" in amd64) other=arm64; cross=aarch64-linux-gnu ;; arm64) other=amd64; cross=x86-64-linux-gnu ;; esac; \
    dpkg --add-architecture "$other"; apt-get update; \
    apt-get install -y --no-install-recommends pkg-config \
        "gcc-$cross" "g++-$cross" \
        libasound2-dev libudev-dev "libasound2-dev:$other" "libudev-dev:$other" \
        mingw-w64; \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-gnu

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
    CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc \
    PKG_CONFIG_ALLOW_CROSS=1
