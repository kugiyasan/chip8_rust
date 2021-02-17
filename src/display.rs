pub struct Display {
    pub width: u32,
    pub height: u32,
    pub screen: [[bool; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        println!("{}", "\n".repeat(32 + 1));
        Self {
            width: 64,
            height: 32,
            screen: [[false; 64]; 32],
        }
    }

    pub fn print_screen(&self) {
        let data = self
            .screen
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| if cell { "@@" } else { ".." })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Move the cursor up to overwrite the previous frame
        // println!("\u{001b}[{}A", self.height + 1);
        println!("{}", data);
    }

    pub fn clear(&mut self) {
        self.screen = [[false; 64]; 32];
    }
}
