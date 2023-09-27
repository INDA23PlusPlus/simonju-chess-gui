mod state;

use state::*;

use ggez::{*, conf::WindowSetup};

fn main() -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("chess", "simonju")
        .add_resource_path(std::path::PathBuf::from("./config"))
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()?;

    let state = State::new(&mut context)?;
    
    event::run(context, event_loop, state);
}
