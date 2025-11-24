use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let a = Notification::new()
        .summary("vs code")
        .body("This will almost look like a real vs code notification.")
        .icon("code")
        .show()?;
   
   println!("{:?}", a);
    Ok(())
}
