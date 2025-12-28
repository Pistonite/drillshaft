use shaft_cli::CliApi;
#[cu::cli(preprocess = CliApi::preprocess)]
async fn main(cli: CliApi) -> cu::Result<()> {
    cli.run().await
}
