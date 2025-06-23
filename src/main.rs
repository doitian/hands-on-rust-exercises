use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.show_menu(ctx),
            GameMode::Playing => self.play_game(ctx),
            GameMode::End => self.end_game(ctx),
        }
    }
}

impl State {
    fn show_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Welcome to the Game!");
        ctx.print(1, 2, "Press 'P' to Play or 'Q' to Quit");
        if ctx.key == Some(VirtualKeyCode::P) {
            self.mode = GameMode::Playing;
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }

    fn play_game(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "You are now playing the game!");
        ctx.print(1, 2, "Press 'E' to End Game or 'Q' to Quit");
        if ctx.key == Some(VirtualKeyCode::E) {
            self.mode = GameMode::End;
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }

    fn end_game(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Game Over!");
        ctx.print(1, 2, "Press 'M' to return to Menu or 'Q' to Quit");
        if ctx.key == Some(VirtualKeyCode::M) {
            self.mode = GameMode::Menu;
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Lib Example")
        .build()?;
    main_loop(context, State::new())
}
