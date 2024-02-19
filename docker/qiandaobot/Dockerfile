FROM rust:1.75 AS builder
WORKDIR /app

COPY . .
RUN make release

FROM alpine:3
WORKDIR /app

COPY --from=builder /app/target/release/qiandaobot /app/qiandaobot

ENTRYPOINT ["./qiandaobot"]
