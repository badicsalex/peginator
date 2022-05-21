#/usr/bin/env sh
cargo run \
    --bin peginator-compile -- \
    -p crate \
    grammar.ebnf \
    >src/grammar/generated2.rs &&\
    mv src/grammar/generated2.rs src/grammar/generated.rs &&\
    rustfmt src/grammar/generated.rs
