use crate::{
    cmd::{read, run},
    config::Config,
};
use anyhow::Result;

pub fn bucket_exists(bucket_name: &str, config: &Config) -> Result<()> {
    let all_buckets = read(&format!("aws s3 ls --output text"), config)?;

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
            "aws s3api put-bucket-policy --bucket {} --policy {}",
            bucket_name, policy
        ),
        config,
    )?;

    Ok(())
}
