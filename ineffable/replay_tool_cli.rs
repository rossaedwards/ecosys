use structopt::StructOpt;

#[derive(StructOpt)]
struct ReplayCli {
    #[structopt(long)]
    log_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = ReplayCli::from_args();
    let events = EventLog::replay(&args.log_path)?;
    println!("Replayed {} events", events.len());
    for event in events {
        println!("{:?}", event);
    }
    Ok(())
}