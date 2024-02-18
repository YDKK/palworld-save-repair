FROM rust:latest AS builder

WORKDIR /work
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /work/target/release/pal_world_save_repair .
RUN chmod +x ./pal_world_save_repair
ENTRYPOINT [ "./pal_world_save_repair" ]