FROM rust:latest as build
COPY . .
RUN cargo build --release

FROM ubuntu:latest
EXPOSE 3000
COPY --from=build /target/release/obct /obct
ENTRYPOINT /obct
CMD /obct
