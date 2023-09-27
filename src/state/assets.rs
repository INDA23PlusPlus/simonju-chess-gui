use ggez::*;

use olindba_chess::*;

pub(crate) struct Assets {
    white_pawn: graphics::Image,
    white_bishop: graphics::Image,
    white_knight: graphics::Image,
    white_rook: graphics::Image,
    white_queen: graphics::Image,
    white_king: graphics::Image,
    black_pawn: graphics::Image,
    black_bishop: graphics::Image,
    black_knight: graphics::Image,
    black_rook: graphics::Image,
    black_queen: graphics::Image,
    black_king: graphics::Image,
}

impl Assets {
    pub(crate) fn new(context: &mut Context) -> GameResult<Assets> {
        let white_pawn = graphics::Image::from_path(context,    "/white_pawn.png")?;
        let white_bishop = graphics::Image::from_path(context,  "/white_bishop.png")?;
        let white_knight = graphics::Image::from_path(context,  "/white_knight.png")?;
        let white_rook = graphics::Image::from_path(context,    "/white_rook.png")?;
        let white_queen = graphics::Image::from_path(context,   "/white_queen.png")?;
        let white_king = graphics::Image::from_path(context,    "/white_king.png")?;
        
        let black_pawn = graphics::Image::from_path(context,    "/black_pawn.png")?;
        let black_bishop = graphics::Image::from_path(context,  "/black_bishop.png")?;
        let black_knight = graphics::Image::from_path(context,  "/black_knight.png")?;
        let black_rook = graphics::Image::from_path(context,    "/black_rook.png")?;
        let black_queen = graphics::Image::from_path(context,   "/black_queen.png")?;
        let black_king = graphics::Image::from_path(context,    "/black_king.png")?;

        Ok(Assets {
            white_pawn,
            white_bishop,
            white_knight,
            white_rook,
            white_queen,
            white_king,
            black_pawn,
            black_bishop,
            black_knight,
            black_rook,
            black_queen,
            black_king,
        })
    }

    pub(crate) fn get_image_from_piece(&self, piece: Piece) -> GameResult<&graphics::Image>{
        match piece.get_color() {
            WHITE => match piece.get_type() {
                PAWN => Ok(&self.white_pawn),
                BISHOP => Ok(&self.white_bishop),
                KNIGHT => Ok(&self.white_knight),
                ROOK => Ok(&self.white_rook),
                QUEEN => Ok(&self.white_queen),
                KING => Ok(&self.white_king),    
                x => Err(GameError::CustomError(format!("Invalid piece type: {}", x))),
            }
            BLACK => match piece.get_type() {
                PAWN => Ok(&self.black_pawn),
                BISHOP => Ok(&self.black_bishop),
                KNIGHT => Ok(&self.black_knight),
                ROOK => Ok(&self.black_rook),
                QUEEN => Ok(&self.black_queen),
                KING => Ok(&self.black_king),    
                x => Err(GameError::CustomError(format!("Invalid piece type: {}", x))),
            },
            x => Err(GameError::CustomError(format!("Invalid piece color: {}", x))),
        }
    }
}