use x_deploy_common::event::user::UserRegisteredEvent;
use x_deploy_common::{CommonError, CommonResult};

pub fn listen_user_registered(event: UserRegisteredEvent) -> CommonResult<()> {
  println!("User registered: {:?}", event);
  Ok(())
}
