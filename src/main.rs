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
    #[clap(subcommand)]
    Env(awsx::env::Subcommands),

    #[clap(subcommand)]
    Stack(awsx::stack::Subcommands),

    #[clap(subcommand)]
    Ec2(awsx::ec2::Subcommands),

    #[clap(subcommand)]
    Bucket(awsx::bucket::Subcommands),

    #[clap(subcommand)]
    Lambda(awsx::lambda::Subcommands),

    #[clap(subcommand)]
    Route53(awsx::route53::Subcommands),

    #[clap(subcommand)]
    Secrets(awsx::secrets::Subcommands),
}

#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = Config::from_path(args.config, Default::default())?;

    match args.cmd {
        Subcommands::Env(cmd) => match cmd {
            awsx::env::Subcommands::Substitute { file, output } => {
                awsx::env::substitute_env_vars(file, output, &config)
            }
            awsx::env::Subcommands::Print {} => awsx::env::print_env_vars(&config),
        },

        Subcommands::Stack(cmd) => match cmd {
            awsx::stack::Subcommands::Create {
                stack_name,
                template,
            } => awsx::stack::create(stack_name, template, &config),
            awsx::stack::Subcommands::Update {
                stack_name,
                template,
            } => awsx::stack::update(stack_name, template, &config),
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
                tag,
            } => awsx::ec2::create_image(name, instance_id, description, tag, &config),
            awsx::ec2::Subcommands::GetLatestAMI { filter, with_name } => {
                awsx::ec2::get_latest_ami(filter, with_name, &config)
            }
        },

        Subcommands::Bucket(cmd) => match cmd {
            awsx::bucket::Subcommands::Cp {
                from,
                to,
                recursive,
            } => awsx::bucket::cp(from, to, recursive, &config),
            awsx::bucket::Subcommands::Rm { path, recursive } => {
                awsx::bucket::rm(path, recursive, &config)
            }
            awsx::bucket::Subcommands::Exists { bucket_name } => {
                awsx::bucket::bucket_exists(&bucket_name, &config)
            }
            awsx::bucket::Subcommands::PutBucketPolicy {
                bucket_name,
                policy,
            } => awsx::bucket::put_bucket_policy(&bucket_name, &policy, &config),
            awsx::bucket::Subcommands::Upload { path, to } => {
                awsx::bucket::upload(path, to, &config)
            }
        },

        Subcommands::Lambda(cmd) => match cmd {
            awsx::lambda::Subcommands::UpdateFunction {
                function_name,
                zip_file,
            } => awsx::lambda::update_function(function_name, zip_file, &config),
        },

        Subcommands::Route53(cmd) => match cmd {
            awsx::route53::Subcommands::HostedZoneId { hosted_zone_name } => {
                awsx::route53::hosted_zone_id(hosted_zone_name, &config)
            }
        },
        Subcommands::Secrets(cmd) => match cmd {
            awsx::secrets::Subcommands::Get { name, key } => awsx::secrets::get(name, key, &config),
        },
    }?;

    Ok(())
}
