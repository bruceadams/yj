FROM bruceadams/rustup-musl as build

COPY --chown=rust:rust . /home/rust
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch as yj

COPY --from=build /home/rust/target/x86_64-unknown-linux-musl/release/yj /

ENTRYPOINT [ "/yj" ]
