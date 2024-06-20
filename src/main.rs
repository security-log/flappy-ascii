use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    /// Constructor
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy ASCII");
        ctx.print_centered(8, "(S) Start a new game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("Total score: {}", self.score));
        ctx.print_centered(8, "(S) Start a new game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // TODO: Add some option to enable debug info.
        // NOTE: This print is for debug, delete later.
        ctx.print(
            0,
            2,
            &format!(
                "player Y:{} | gravity:{}",
                self.player.y, self.player.velocity
            ),
        );

        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    /// Starts a new game, resetting the game state and indicating that the game is in progress.
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

struct Player {
    // TODO: Make player coordinates flotar for smoother movement.
    x: i32, // This represent the progress through the lvl
    y: i32,
    velocity: f32, // Vertical momentum
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, WHEAT2, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        // Increment gravity
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        if self.y < 0 {
            // Handle the case in case the player go outside the frame, 0 is the top.
            self.y = 0;
        }

        // Distance in world-space.
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32, // this is the location in x world-space
    gap_y: i32,
    gap_size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.gap_size / 2;

        // Draw the top half
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, WHEAT1, BLACK, to_cp437('#'));
        }

        // Draw the bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, WHEAT1, BLACK, to_cp437('#'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.gap_size / 2;
        let does_x_match = player.x == self.x;

        let player_above_gap = player.y < self.gap_y - half_size;
        let player_under_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_under_gap || player_above_gap)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy ASCII")
        .build()?;

    main_loop(context, State::new())
}
