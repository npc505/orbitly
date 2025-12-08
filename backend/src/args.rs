#[derive(clap::Parser)]
pub struct Args {
    #[clap(short, long, env = "PORT")]
    pub port: u16,
    #[clap(short, long, env = "ADDRESS")]
    pub address: std::net::IpAddr,
    #[clap(flatten)]
    pub neo4j: Neo4j,
}

#[derive(clap::Parser)]
pub struct Neo4j {
    #[clap(long, env = "NEO_URI")]
    pub neo_uri: axum::http::Uri,
    #[clap(long, env = "NEO_PASSWORD")]
    pub neo_password: String,
    #[clap(long, env = "NEO_USERNAME")]
    pub neo_username: String,
}

impl Neo4j {
    pub fn to_config(self) -> neo4rs::Result<neo4rs::Config> {
        neo4rs::ConfigBuilder::new()
            .uri(self.neo_uri.to_string())
            .user(self.neo_username)
            .password(self.neo_password)
            .build()
    }
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
