// TODO: Add a not about how to use `GIT_DIR`

pub(crate) mod cli {
    use {
        // clap_verbosity_flag::Verbosity,
        structopt::{
            clap::{
                _clap_count_exprs,
                arg_enum,
            },
            StructOpt,
        },
    };

    #[derive(Debug, StructOpt)]
    #[structopt(rename_all = "snake_case")]
    pub(crate) struct Args {
        // #[structopt(flatten)]
        // verbosity: Verbosity,
        #[structopt(raw(possible_values = "&ServiceName::variants()", case_insensitive = "true"))]
        service: ServiceName,
        #[structopt(subcommand)]
        subcommand: Subcommand,
    }

    arg_enum! {
        #[derive(Debug)]
        enum ServiceName {
            Github,
            // Gitlab, // TODO: Look into this!
            // BitBucket, // TODO: Look into this!
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(rename_all = "snake_case")]
    pub(crate) enum Subcommand {
        Fork,
        New,
        Delete,
        // Clone, // FIXME: I definitely am interested in exploring this
        // Open, // FIXME: I definitely am interested in exploring this.
    }
}

use structopt::StructOpt;

pub mod services {
    pub enum ServiceInitError {

    }

    pub enum ForkRepoError {

    }

    pub enum NewRepoError {

    }

    pub enum DeleteRepoError {

    }

    trait Service {
        fn from_cwd() -> Result<Self, ServiceInitError>;
        fn fork_repo(&self, repo: RepositoryRef) -> Result<(), ForkRepoError>;
        fn new_repo(&self, repo: RepositoryRef) -> Result<(), NewRepoError>;
        fn delete_repo(&self, repo: RepositoryRef) -> Result<(), DeleteRepoError>;
    }

    pub mod github {
        struct Github;

        impl Service for Github {

        }
    }
}

fn main() {
    let args = self::cli::Args::from_args();
    println!("args: {:#?}", args);
}
