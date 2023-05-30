use crate::{cmd::read, config::Config};
use anyhow::Result;

pub fn hosted_zone_id(hosted_zone_name: impl AsRef<str>, config: &Config) -> Result<()> {
    let hosted_zone = read(
        &format!(
            "aws route53 list-hosted-zones-by-name --dns-name {} --output text --query 'HostedZones[?Name==`{}.`].Id'",
            hosted_zone_name.as_ref(),
            hosted_zone_name.as_ref(),
        ),
        config,
    )?;
    let hosted_zone = hosted_zone.replace("/hostedzone/", "");

    println!("{}", hosted_zone);

    Ok(())
}
