use awsx::config::Config;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "awsx", about = "Opinionated wrapper around the AWS CLI")]
pub struct Args {
    /// Path to 'config.toml' file. Will scan every file with the same name up to the project root.
    #[clap(long, short = 'c')]
    config: PathBuf,

    /// The default value is the first parent folder containing a .git folder.
    #[clap(long, short = 'p')]
    project_root: Option<PathBuf>,

    /// Just print the command(s) that would run instead of actually running them.
    #[clap(long, short = 'n')]
    dry_run: Option<PathBuf>,

    #[clap(subcommand)]
    cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Replace all occurences of existing environment variables in a file.
    /// By default the result will be printed to stdout.
    Substitute {
        /// Path to file in which all occurences of existing env vars will be replaced.
        file: PathBuf,

        /// Optionally write the output to a file instead of printing to stdout.
        #[clap(long, short = 'o')]
        output: Option<PathBuf>,
    },

    #[clap(subcommand)]
    Stack(awsx::stack::Subcommands),

    #[clap(subcommand)]
    Ec2(awsx::ec2::Subcommands),

    #[clap(subcommand)]
    Bucket(awsx::bucket::Subcommands),

    #[clap(subcommand)]
    Lambda(awsx::lambda::Subcommands),
}

#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut config = Config::from_path(args.config, Default::default())?;

    match args.cmd {
        Subcommands::Substitute { file, output } => {
            awsx::substitute::substitute_env_vars(file, output, &config)
        }

        Subcommands::Stack(cmd) => match cmd {
            awsx::stack::Subcommands::Create {
                stack_name,
                template,
            } => awsx::stack::create(stack_name, template, &mut config),
            awsx::stack::Subcommands::Destroy { stack_name } => {
                awsx::stack::destroy(stack_name, &config)
            }
            awsx::stack::Subcommands::Output {
                stack_name,
                output_name,
            } => awsx::stack::output(stack_name, output_name, &config),
            awsx::stack::Subcommands::Validate { template } => {
                awsx::stack::validate(template, &config)
            }
        },

        Subcommands::Ec2(cmd) => match cmd {
            awsx::ec2::Subcommands::CreateInstance { options } => {
                awsx::ec2::create_instance(options, &config)
            }
            awsx::ec2::Subcommands::StartInstance { instance_id } => {
                awsx::ec2::start_instance(instance_id, &config)
            }
            awsx::ec2::Subcommands::StopInstance { instance_id } => {
                awsx::ec2::stop_instance(instance_id, &config)
            }
            awsx::ec2::Subcommands::CreateImage {
                name,
                instance_id,
                description,
            } => awsx::ec2::create_image(name, instance_id, description, &config),
            awsx::ec2::Subcommands::GetLatestAMI { filter, with_name } => {
                awsx::ec2::get_latest_ami(filter, with_name, &config)
            }
        },

        Subcommands::Bucket(cmd) => match cmd {
            awsx::bucket::Subcommands::Exists { bucket_name } => {
                awsx::bucket::bucket_exists(&bucket_name, &config)
            }
            awsx::bucket::Subcommands::PutBucketPolicy {
                bucket_name,
                policy,
            } => awsx::bucket::put_bucket_policy(&bucket_name, &policy, &config),
        },

        Subcommands::Lambda(cmd) => match cmd {
            awsx::lambda::Subcommands::UpdateFunction {
                function_name,
                zip_file,
            } => awsx::lambda::update_function(function_name, zip_file, &config),
        },
    }?;

    Ok(())
}
