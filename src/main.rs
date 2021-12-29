use nats;

fn main() -> std::io::Result<()> {
  let nc = nats::connect("jarvis-nats")?;

  let sub = nc.subscribe("jarvis")?;
  for msg in sub.messages() {}

  Ok(())
}
