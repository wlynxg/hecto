use buffer::Buffer;

use crate::editor::terminal::{Size, Terminal};

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit too far up or down
        #[allow(clippy::integer_division)]
            let vertical_center = height / 3;

        for current in 0..height {
            if let Some(line) = self.buffer.lines.get(current) {
                let truncated_line = if line.len() >= width { &line[0..width] } else { line };
                Self::render_line(current, truncated_line);
            } else if current == vertical_center && self.buffer.is_empty() {
                Self::render_line(current, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current, "~");
            }
            self.needs_redraw = false;
        }
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 { return " ".to_string(); }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
            let padding = (width.saturating_sub(len).saturating_add(1)) >> 1;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
