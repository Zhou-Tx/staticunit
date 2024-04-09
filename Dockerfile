FROM rust:alpine AS Builder

RUN apk add --no-cache musl-dev

WORKDIR /opt
COPY . .
RUN cargo build --release

### Runner ###
FROM gcr.io/distroless/static

WORKDIR /app
COPY --from=Builder /opt/target/release/staticunit /sbin/

CMD ["staticunit"]
EXPOSE 80
