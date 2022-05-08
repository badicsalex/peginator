fn main() {
    peginator::buildscript::Compile::directory("src")
        .format()
        .run()
        .unwrap();
}
