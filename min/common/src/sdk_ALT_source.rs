use sdk::process::Send;

fn sdk() {
  let alt = process::new("SenderArg")
  .alt("source")
  .exec("<sdk>R")
  .func("Sender")
}