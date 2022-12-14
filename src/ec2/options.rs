/// Commands that control Cloud Formation related tasks
#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Creates an EC2 instance
    CreateInstance {
        /// todo
        #[clap(flatten)]
        options: CreateInstanceOptions,
    },

    /// Creates an Aamazon Machine Image from a running instance
    CreateImage {
        /// The intended name for the AMI
        name: String,

        /// ID of the instance to create an image from
        #[clap(long, short = 'i')]
        instance_id: String,

        /// Description for the AMI
        #[clap(long, short = 'd')]
        description: Option<String>,
    },

    /// Get the id of the latest AMI
    GetLatestAMI {
        /// Filter that is applied to the name of the AMIs
        #[clap(long, short = 'f')]
        filter: Option<String>,

        /// Output the name along with the id
        #[clap(long, action)]
        with_name: bool,
    },
}

/// Doc comment
#[derive(clap::Args, Debug)]
pub struct CreateInstanceOptions {
    /// How many instances of this type to spawn
    #[clap(long, default_value = "1")]
    pub count: u8,

    /// The name of the keypair to associate with this instance
    #[clap(long)]
    pub keypair: Option<String>,

    /// Image id to boot the instance from
    #[clap(long)]
    pub image_id: String,

    /// EC2 instane type
    #[clap(long)]
    pub instance_type: String,

    /// todo
    #[clap(long, default_value = "32")]
    pub volume_size: u8,

    /// A single security group id. Argument can be used multiple times
    #[clap(long)]
    pub security_group_id: Vec<String>,

    /// String of security group ids separated by a whitesingle space
    #[clap(long)]
    pub security_group_ids: Option<String>,

    /// Script that runs on instance startup
    #[clap(long)]
    pub user_data: Option<String>,

    /// A single tag to assign to the instance. Argument can be used multiple times.
    #[clap(long)]
    pub tag: Vec<String>,

    /// IAM instance profile name
    #[clap(long)]
    pub instance_profile: Option<String>,
}
