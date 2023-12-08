use crate::{
    cmd::{read, run},
    config::Config,
};
use anyhow::Result;

pub fn bucket_exists(bucket_name: &str, config: &Config) -> Result<()> {
    let all_buckets = read("aws s3 ls --output text", config)?;

    let bucket_exists = all_buckets
        .lines()
        .any(|line| match line.split_whitespace().nth(2) {
            Some(name) => name.eq(bucket_name),
            None => false,
        });

    println!("{}", bucket_exists);

    Ok(())
}

pub fn put_bucket_policy(bucket_name: &str, policy: &str, config: &Config) -> Result<()> {
    run(
        &format!(
            "aws s3api put-bucket-policy --bucket {} --policy '{}'",
            bucket_name, policy
        ),
        config,
    )?;

    Ok(())
}

pub fn cp(
    from: impl AsRef<str>,
    to: impl AsRef<str>,
    recursive: bool,
    config: &Config,
) -> Result<()> {
    let recursive = if recursive { "--recursive" } else { "" };

    run(
        &format!("aws s3 cp {} {} {}", recursive, from.as_ref(), to.as_ref()),
        config,
    )?;

    Ok(())
}

pub fn rm(path: impl AsRef<str>, recursive: bool, config: &Config) -> Result<()> {
    let recursive = if recursive { "--recursive" } else { "" };

    run(
        &format!("aws s3 rm {} {}", recursive, path.as_ref()),
        config,
    )?;

    Ok(())
}

pub fn upload(path: impl AsRef<str>, to: impl AsRef<str>, config: &Config) -> Result<()> {
    let timestamp = read("date +\"%Y-%m-%d_%H:%M:%S\"", &config)?;
    let to = to.as_ref().trim_end_matches('/');

    cp(path, format!("{to}/{timestamp}/"), true, config)?;
    rm(format!("{to}/latest/",), true, config)?;
    cp(
        format!("{to}/{timestamp}/"),
        format!("{to}/latest/"),
        true,
        config,
    )?;

    Ok(())
}
