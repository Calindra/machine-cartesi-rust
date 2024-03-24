# syntax=docker.io/docker/dockerfile:1.6

FROM rust:1.75.0-bookworm as builder

# https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/reference.md#example-cache-apt-packages
RUN <<EOF
rm -f /etc/apt/apt.conf.d/docker-clean
echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache
EOF

RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  g++-riscv64-linux-gnu=4:12.2.0-5 \
  protobuf-compiler=3.21.12-3

RUN rustup target add riscv64gc-unknown-linux-gnu

WORKDIR /usr/src/app

COPY . ./

# https://docs.docker.com/build/cache/#use-the-dedicated-run-cache
# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry/,sharing=locked \
  cargo build --release --target=riscv64gc-unknown-linux-gnu

# Change from playground to ubuntu with riskv64

FROM cartesi/playground:0.6.0 as playground

WORKDIR /usr/src/app
RUN mkdir machine

COPY --from=builder /usr/src/app/target/riscv64gc-unknown-linux-gnu/release/machine-cartesi-rust machine/

RUN tar \
  --sort=name \
  --mtime='2024-01-01' \
  --owner=1000 \
  --group=1000 \
  --numeric-owner \
  -cf machine.tar \
  --directory=machine .

RUN genext2fs \
  -f \
  -b 16384 \
  -a machine.tar \
  machine.ext2 \
  && rm -rf machine.tar

CMD ["cartesi-machine" "--flash-drive" "label:machine,filename:machine.ext2", "--", "/mnt/machine-cartesi-rust"]
