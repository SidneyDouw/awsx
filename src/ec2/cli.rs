use super::options::CreateInstanceOptions;
use crate::{
    cmd::{read, run},
    config::Config,
};
use anyhow::Result;

pub fn create_instance(options: CreateInstanceOptions, config: &Config) -> Result<()> {
    let CreateInstanceOptions {
        count,
        keypair,
        image_id,
        instance_type,
        volume_type,
        volume_size,
        security_group_id: security_group_id_vec,
        security_group_ids,
        user_data,
        tag: tags,
        instance_profile,
    } = options;

    let mut cmd_str = String::from("aws ec2 run-instances ");
    cmd_str.push_str(&format!("--count {} ", count));
    keypair.map(|keypair| cmd_str.push_str(&format!("--key-name {} ", keypair)));
    cmd_str.push_str(&format!("--image-id {} ", image_id));
    cmd_str.push_str(&format!("--instance-type {} ", instance_type));
    cmd_str.push_str(&format!("--block-device-mappings \"DeviceName=/dev/sda1, Ebs={{VolumeType={}, VolumeSize={}, Iops=1000, DeleteOnTermination=true }}\" ", volume_type, volume_size));
    cmd_str.push_str(&format!("--associate-public-ip-address "));

    if let Some(security_group_ids) = security_group_ids {
        cmd_str.push_str(&format!("--security-group-ids {} ", security_group_ids));
    } else {
        if !security_group_id_vec.is_empty() {
            cmd_str.push_str(&format!("--security-group-ids "));
            for id in security_group_id_vec {
                cmd_str.push_str(&format!("{} ", id));
            }
        }
    }

    user_data.map(|user_data| cmd_str.push_str(&format!("--user-data {} ", user_data)));

    if !tags.is_empty() {
        cmd_str.push_str(&format!(
            "--tag-specifications \"ResourceType=instance, Tags=["
        ));
        for t in tags {
            let kvpair = t.split(',').collect::<Vec<_>>();
            cmd_str.push_str(&format!("{{Key={},Value={}}},", kvpair[0], kvpair[1]));
        }
        cmd_str.push_str(&format!("]\" "));
    }

    instance_profile.map(|instance_profile| {
        cmd_str.push_str(&format!(
            "--iam-instance-profile \"Name={}\" ",
            instance_profile
        ))
    });

    let instance_info = read(&cmd_str, config)?;

    println!("Creating Instance with instance_id");
    println!("{}", instance_info);
    println!("Waiting for completion...");

    // run_cmd(format!("aws ec2 wait instance-status-ok --instance-ids {}", instance_id);

    println!("Done");

    Ok(())
}

pub fn create_image(
    name: String,
    instance_id: String,
    description: Option<String>,
    config: &Config,
) -> Result<()> {
    let cmd = format!(
        "aws ec2 create-image --name {} --instance-id {} --output text",
        name, instance_id,
    );
    let cmd = match description {
        Some(description) => format!("{cmd} --description \"{description}\""),
        _ => cmd,
    };

    let image_id = read(&cmd, config)?;

    println!("Creating AMI {:?} with image_id {:?}", name, image_id);
    println!("Waiting for completion...");

    run(
        &format!("aws ec2 wait image-available --image-ids {}", image_id),
        config,
    )?;

    println!("Done");

    Ok(())
}

pub fn get_latest_ami(filter: Option<String>, with_name: bool, config: &Config) -> Result<()> {
    let cmd = format!("aws ec2 describe-images --owners self --query \"Images[].[CreationDate, Name, ImageId] | sort_by(@, &[0])\" --output text");
    let out = read(&cmd, config)?;
    let latest_ami = out
        .lines()
        .filter_map(|line| {
            let s = line.split('\t').collect::<Vec<_>>();
            let name = *s.get(1).expect("name present");
            let id = *s.get(2).expect("id present");
            let out = match with_name {
                true => format!("{}\t{}", name, id),
                false => id.to_owned(),
            };
            match &filter {
                Some(filter) => name.contains(filter).then(|| out),
                None => Some(out),
            }
        })
        .last()
        .ok_or(anyhow::anyhow!("No AMI found"))?;

    println!("{}", latest_ami);

    Ok(())
}
