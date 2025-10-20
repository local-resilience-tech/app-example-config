# BACKEND BUILDER
FROM --platform=$BUILDPLATFORM rust:1 AS rustbuilder
ARG TARGETARCH
WORKDIR /app

# should write /.platform and /.compiler
COPY --chmod=555 deployment/platform.sh .
RUN ./platform.sh

# setup rust compilation for the target platform
RUN rustup component add rustfmt
CMD /bin/bash
RUN rustup target add $(cat /app/.platform)
RUN apt-get update && apt-get install -y unzip $(cat /app/.compiler) pkg-config libssl-dev
COPY deployment/cargo-config.toml ./.cargo/config

# Compile the backend
COPY . .
RUN RUSTFLAGS=-g cargo build --release --target $(cat /app/.platform)
RUN cp /app/target/$(cat /app/.platform)/release/example-config /app/example-config

# RUNNER
FROM ubuntu AS runner
COPY --from=rustbuilder /app/example-config /app/backend/example-config
EXPOSE 8200
WORKDIR /app/backend
CMD ["./example-config"]