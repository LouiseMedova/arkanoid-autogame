use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

fn check_circle_rectangle_collision(
    circle_x: Decimal,
    circle_y: Decimal,
    radius: Decimal,
    rect_x1: Decimal,
    rect_y1: Decimal,
    rect_x2: Decimal,
    rect_y2: Decimal,
) -> Option<(bool, bool)> {
    let nearest_x = rect_x1.max(circle_x.min(rect_x2));
    let nearest_y = rect_y1.max(circle_y.min(rect_y2));

    let distance_x = circle_x - nearest_x;
    let distance_y = circle_y - nearest_y;
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let radius_squared = radius * radius;

    if distance_squared <= radius_squared {
        let collision_x = nearest_x == rect_x1 || nearest_x == rect_x2;
        let collision_y = nearest_y == rect_y1 || nearest_y == rect_y2;
        Some((collision_x, collision_y))
    } else {
        None
    }
}

struct Block {
    rect_x1: f32,
    rect_y1: f32,
    rect_x2: f32,
    rect_y2: f32,
    is_visible: bool, 
}

impl Block {
    fn new(x1: f32, y1: f32, width: f32, height: f32) -> Self {
        Block {
            rect_x1: x1,
            rect_y1: y1,
            rect_x2: x1 + width,
            rect_y2: y1 + height,
            is_visible: true,
        }
    }
}

struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
    direction: f32,
}

impl Paddle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Paddle {
            x,
            y,
            width,
            height,
            speed: 5.0,    
            direction: 1.0, 
        }
    }

    fn update_position(&mut self) {
        self.x += self.speed * self.direction;

        if self.x <= 0.0 || self.x + self.width >= 800.0 {
            self.direction = -self.direction; 
        }
    }
}

struct MainState {
    circle_x: f32,
    circle_y: f32,
    radius: f32,
    velocity_x: f32,
    velocity_y: f32,
    blocks: Vec<Block>,
    paddle: Paddle,
}

impl MainState {
    fn new() -> Self {
        let mut blocks = Vec::new();
        let block_width = 30.0;
        let block_height = 30.0;
        let rows = 5;
        let cols = 10;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * (block_width + 5.0); 
                let y = row as f32 * (block_height + 5.0);
                blocks.push(Block::new(x, y, block_width, block_height));
            }
        }

        let paddle = Paddle::new(375.0, 550.0, 400.0, 10.0);

        MainState {
            circle_x: 400.0,
            circle_y: 300.0,
            radius: 15.0,
            velocity_x: 3.0,
            velocity_y: 3.0,
            blocks,
            paddle,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.circle_x += self.velocity_x;
        self.circle_y += self.velocity_y;

        self.paddle.update_position();

        if self.circle_x - self.radius <= 0.0 || self.circle_x + self.radius >= 800.0 {
            self.velocity_x = -self.velocity_x;
        }
        if self.circle_y - self.radius <= 0.0 || self.circle_y + self.radius >= 600.0 {
            self.velocity_y = -self.velocity_y;
        }

        if self.circle_y + self.radius >= self.paddle.y
            && self.circle_x >= self.paddle.x
            && self.circle_x <= self.paddle.x + self.paddle.width
        {
            self.velocity_y = -self.velocity_y;

            let paddle_center = self.paddle.x + (self.paddle.width / 2.0);
            let distance_from_center = self.circle_x - paddle_center;

            // If circle is far from center we increase its speed
            self.velocity_x += distance_from_center * 0.05;
        }

        for block in self.blocks.iter_mut() {
            if block.is_visible {
                if let Some((collision_x, collision_y)) = check_circle_rectangle_collision(
                    Decimal::from_f32(self.circle_x).unwrap(),
                    Decimal::from_f32(self.circle_y).unwrap(),
                    Decimal::from_f32(self.radius).unwrap(),
                    Decimal::from_f32(block.rect_x1).unwrap(),
                    Decimal::from_f32(block.rect_y1).unwrap(),
                    Decimal::from_f32(block.rect_x2).unwrap(),
                    Decimal::from_f32(block.rect_y2).unwrap(),
                ) {
                    if collision_x {
                        self.velocity_x = -self.velocity_x;
                    }
                    if collision_y {
                        self.velocity_y = -self.velocity_y;
                    }

                    block.is_visible = false;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2 {
                x: self.circle_x,
                y: self.circle_y,
            },
            self.radius,
            2.0,
            Color::BLUE,
        )?;
        graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;

        for block in &self.blocks {
            if block.is_visible {
                let rect = graphics::Rect::new(
                    block.rect_x1,
                    block.rect_y1,
                    block.rect_x2 - block.rect_x1,
                    block.rect_y2 - block.rect_y1,
                );
                let rectangle = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
                graphics::draw(ctx, &rectangle, (Point2 { x: 0.0, y: 0.0 },))?;
            }
        }

        let paddle_rect = graphics::Rect::new(
            self.paddle.x,
            self.paddle.y,
            self.paddle.width,
            self.paddle.height,
        );
        let paddle = Mesh::new_rectangle(ctx, DrawMode::fill(), paddle_rect, Color::RED)?;
        graphics::draw(ctx, &paddle, (Point2 { x: 0.0, y: 0.0 },))?;

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("circle_rectangle_collision", "Author")
        .build()
        .expect("Failed to build ggez context");

    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
