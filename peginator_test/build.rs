fn main() {
    peginator::buildscript::Compile::directory("src")
        .format()
        .run_exit_on_error()
}
