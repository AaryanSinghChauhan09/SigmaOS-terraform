use anyhow::Result;
use clap::Parser;
use terraform::terraform;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = terraform::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = terraform::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-terraform deployed");
    Ok(())
}
