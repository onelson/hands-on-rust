use bracket_lib::prelude::*;

struct State;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(1, "Hello Bracket World!");
    }
}

fn main() -> BResult<()> {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .build()?;

    main_loop(ctx, State {})
}
