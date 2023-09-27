impl game_state {
    fn run(&mut self, ctx: &mut Context) -> GameResult {
        let cb = ggez::ContextBuilder::new("super_simple", "ggez");
        let (ctx, event_loop) = cb.build()?;
        event::run(ctx, event_loop, self)
    }
}