#/usr/bin/env sh

OUTPUT="src/grammar/generated.rs"

if cargo run -- grammar.ebnf | rustfmt > $OUTPUT.new; then
    if mv -f $OUTPUT.new $OUTPUT; then
        exit 0
    fi
fi

rm -f $OUTPUT.new
exit 1
