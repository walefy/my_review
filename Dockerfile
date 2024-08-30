FROM rust:1-slim-buster AS build

WORKDIR /app

COPY . .

RUN apt update && apt install pkg-config libssl-dev -y

RUN cargo prisma generate

RUN cargo build \
  --release \
  --target x86_64-unknown-linux-gnu

FROM debian:buster-slim

COPY --from=build /app/target/x86_64-unknown-linux-gnu/release/my_review /app/
COPY --from=build /usr/lib/x86_64-linux-gnu/libssl.so.1.1 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
COPY --from=build /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1
COPY --from=build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1

EXPOSE 3001

CMD ["/app/my_review"]
