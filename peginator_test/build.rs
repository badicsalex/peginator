fn main() {
    peginator::buildscript::Compile::directory("src")
        .format()
        .use_peginator_build_time()
        .run_exit_on_error()
}
