use crate::engine::Program;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(parse(try_from_str = parse_vibe))]
    /// The vibe you're into. Current possibilities: ambient, jazz.
    pub vibe: Program,
}

fn parse_vibe(vibe: &str) -> anyhow::Result<Program> {
    match vibe {
        "ambient" => Ok(Program::Ambient),
        "jazz" => Ok(Program::Jazz),
        _ => Err(anyhow::anyhow!("Invalid vibe: {}", vibe)),
    }
}
