use super::util::{get_parameter_values_from_config, parameters_to_string};
use crate::{
    cmd::{read, run},
    config::Config,
};
use anyhow::Result;
use std::path::Path;

pub fn create(
    stack_name: impl AsRef<str>,
    template: impl AsRef<Path>,
    config: &Config,
) -> Result<()> {
    // TODO: Can we deduplicate some code here regarding the expression evaluation in the config parameters?
    // This work is already being done inside the `run` function
    let parameters = parameters_to_string(get_parameter_values_from_config(&template, config)?);

    let cmd = format!(
            "aws cloudformation create-stack --stack-name {} --template-body file://{} --capabilities CAPABILITY_NAMED_IAM {}",
            stack_name.as_ref(), template.as_ref().to_string_lossy(), parameters
        );
    run(&cmd, config)?;

    println!("Creating stack: {:?}", stack_name.as_ref());
    println!("Waiting for completion...");

    run(
        &format!(
            "aws cloudformation wait stack-create-complete --stack-name {}",
            stack_name.as_ref()
        ),
        config,
    )?;

    println!("Done");

    Ok(())
}

pub fn update(
    stack_name: impl AsRef<str>,
    template: impl AsRef<Path>,
    config: &Config,
) -> Result<()> {
    // TODO: Can we deduplicate some code here regarding the expression evaluation in the config parameters?
    // This work is already being done inside the `run` function
    let parameters = parameters_to_string(get_parameter_values_from_config(&template, config)?);

    let cmd = format!(
            "aws cloudformation update-stack --stack-name {} --template-body file://{} --capabilities CAPABILITY_NAMED_IAM {}",
            stack_name.as_ref(), template.as_ref().to_string_lossy(), parameters
        );
    run(&cmd, config)?;

    println!("Updating stack: {:?}", stack_name.as_ref());
    println!("Waiting for completion...");

    run(
        &format!(
            "aws cloudformation wait stack-update-complete --stack-name {}",
            stack_name.as_ref()
        ),
        config,
    )?;

    println!("Done");

    Ok(())
}

pub fn destroy(stack_name: impl AsRef<str>, config: &Config) -> Result<()> {
    run(
        &format!(
            "aws cloudformation delete-stack --stack-name {}",
            stack_name.as_ref()
        ),
        config,
    )?;

    println!("Deleting stack: {:?}", stack_name.as_ref());
    println!("Waiting for completion...");

    run(
        &format!(
            "aws cloudformation wait stack-delete-complete --stack-name {}",
            stack_name.as_ref()
        ),
        config,
    )?;

    println!("Done");

    Ok(())
}

pub fn output(
    stack_name: impl AsRef<str>,
    output_name: Option<impl AsRef<str>>,
    config: &Config,
) -> Result<()> {
    let cmd = format!(
        "aws cloudformation describe-stacks --stack-name {} --output text --query Stacks[0].Outputs[*]",
        stack_name.as_ref()
    );
    let raw_output = read(&cmd, config)?;

    if let Some(output_name) = output_name {
        let value = raw_output
            .lines()
            .filter_map(|line| line.split_once('\t'))
            .find_map(|(k, v)| (k == output_name.as_ref()).then_some(v));

        match value {
            Some(value) => {
                println!("{}", value);
                Ok(())
            }
            None => anyhow::bail!(
                "Could not find output variable {:?} in stack {:?}",
                output_name.as_ref(),
                stack_name.as_ref()
            ),
        }
    } else {
        println!("{}", raw_output);
        Ok(())
    }
}

pub fn validate(template: impl AsRef<Path>, config: &Config) -> Result<()> {
    let cmd = format!(
        "aws cloudformation validate-template --template-body file://{}",
        template.as_ref().to_string_lossy()
    );

    run(&cmd, config)?;

    Ok(())
}
