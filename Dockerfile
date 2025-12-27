FROM rust:1.83 AS builder

WORKDIR /build

RUN curl -L https://github.com/rustwasm/wasm-pack/releases/download/v0.13.1/wasm-pack-v0.13.1-x86_64-unknown-linux-musl.tar.gz \
    | tar -xz -C /usr/local/bin --strip-components=1 wasm-pack-v0.13.1-x86_64-unknown-linux-musl/wasm-pack

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY .cargo ./.cargo

RUN wasm-pack build --target web --out-dir web/pkg

COPY web ./web

FROM nginx:alpine

COPY --from=builder /build/web /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 5000

CMD ["sh", "-c", "sed -i \"s/listen 80;/listen $PORT;/g\" /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
