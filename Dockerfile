FROM rust:1.83 as builder

WORKDIR /build

RUN cargo install wasm-pack

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
