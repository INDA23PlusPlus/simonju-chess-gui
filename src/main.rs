mod state;
mod networking;
mod board;

use state::*;
use ggez::*;

/* Todo
 * There seems to be an issue when promoting pieces letting the player move twice. X
 * I must handle the game state: check, checkmate, fifty rule limit etc. X
 * Castling does not function properly: the lib seems to think a castling move invalid
 * if moving to the first square would have resulted in the king being left in check.
 */

fn main() -> GameResult {
    let (mut context, event_loop) = ContextBuilder::new("chess", "simonju")
        .add_resource_path(std::path::PathBuf::from("./config"))
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()?;

    let state = State::new(&mut context)?;
    
    event::run(context, event_loop, state);
}

