use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Mesh, Text, Font};

pub struct MainMenu {
    buttons: Vec<Button>,
    selected_game: Option<String>,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut buttons = Vec::new();
        buttons.push(Button::new("Flappy Bird", (100.0, 100.0)));
        buttons.push(Button::new("Sneak", (100.0, 160.0)));
        buttons.push(Button::new("Pong", (100.0, 220.0)));
        buttons.push(Button::new("Memory", (100.0, 280.0)));

        MainMenu {
            buttons,
            selected_game: None,
        }
    }

    pub fn is_button_clicked(&mut self, x: f32, y: f32) -> bool {
        for button in &self.buttons {
            if button.is_clicked(x, y) {
                self.selected_game = Some(button.label.clone());
                return true;
            }
        }
        false
    }

    pub fn get_selected_game(&self) -> Option<String> {
        self.selected_game.clone()
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for button in &self.buttons {
            button.draw(ctx)?;
        }
        graphics::present(ctx)
    }
}

pub struct Button {
    label: String,
    position: (f32, f32),
    width: f32,
    height: f32,
}

impl Button {
    pub fn new(label: &str, position: (f32, f32)) -> Self {
        Button {
            label: label.to_string(),
            position,
            width: 200.0,
            height: 50.0,
        }
    }

    pub fn is_clicked(&self, x: f32, y: f32) -> bool {
        x > self.position.0 && x < self.position.0 + self.width
            && y > self.position.1 && y < self.position.1 + self.height
    }


    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let font = Font::default();
        let text = Text::new(&self.label);
        let rect = graphics::Rect::new(self.position.0, self.position.1, self.width, self.height);

        graphics::draw(ctx, &text, (self.position.into(),))?;

        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), rect, Color::WHITE)?;
        graphics::draw(ctx, &mesh, (self.position.into(),))?;

        Ok(())
    }
}
