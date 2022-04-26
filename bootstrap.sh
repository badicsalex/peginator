#/usr/bin/env sh
cargo run \
    --bin peginator-compile -- \
    -p crate \
    peginator/grammar.ebnf \
    >peginator/src/grammar/generated2.rs &&\
    mv peginator/src/grammar/generated2.rs peginator/src/grammar/generated.rs &&\
    rustfmt peginator/src/grammar/generated.rs
