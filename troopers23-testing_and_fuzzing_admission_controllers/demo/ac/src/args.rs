use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Port the validator should run on
    #[arg(short, long, env, default_value = "8443")]
    pub port: u16,

    /// Certificate file path for TLS
    #[arg(long, env, required = true)]
    pub tls_cert_file: String,

    /// Private key file path for certificate for TLS
    #[arg(long, env, required = true)]
    pub tls_key_file: String,
}

pub fn parse() -> Args {
    Args::parse()
}
