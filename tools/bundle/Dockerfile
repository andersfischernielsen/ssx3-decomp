# A Linux build environment for tools/bundle.py (it builds for the OS it runs
# on). Lets a Mac or Windows machine with Docker make and try the Linux build:
#
#   docker build -t ssx3-linux tools/bundle
#   docker run --rm -v "$PWD:/src" -w /src -e CARGO_TARGET_DIR=target/linux \
#       ssx3-linux python3 tools/bundle.py
#
# The repository (with the generated code built from your game) is mounted at
# run time; nothing of the game goes into the image. The build is for the
# container's architecture (aarch64 on Apple Silicon, x86_64 elsewhere).
FROM rust:1-bookworm

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libasound2-dev libudev-dev python3 && \
    rm -rf /var/lib/apt/lists/*

# The generated game's shards each need a lot of memory to compile; Docker's
# VM is often small (4 GB on a Mac by default). Two at a time fit.
ENV CARGO_BUILD_JOBS=2
