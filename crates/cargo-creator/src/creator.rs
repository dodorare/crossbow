fn main() -> anyhow::Result<()> {
    env_logger::init();
    cargo_creator::cli_run("creator")
}
