FROM rust:latest as build
COPY . .
RUN cargo build --release

FROM ubuntu:latest

ARG USERNAME=user-name-goes-here
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
USER $USERNAME

WORKDIR /obct/data
EXPOSE 3000
COPY --from=build /target/release/obct /obct/obct
ENTRYPOINT ../obct
CMD ../obct
