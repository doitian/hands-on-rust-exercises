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
        ctx.print_centered(5, "Welcome to the Game!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if ctx.key == Some(VirtualKeyCode::P) {
            self.mode = GameMode::Playing;
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }

    fn play_game(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are now playing the game!");
        ctx.print_centered(8, "(E) End Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if ctx.key == Some(VirtualKeyCode::E) {
            self.mode = GameMode::End;
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }

    fn end_game(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if ctx.key == Some(VirtualKeyCode::P) {
            self.restart();
        } else if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Lib Example")
        .build()?;
    main_loop(context, State::new())
}
