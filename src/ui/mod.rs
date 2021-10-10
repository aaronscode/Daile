use tcod::console::*;

pub struct FrameBorder {
    top: u32,
    left: u32,
    width: u32,
    height: u32,
    corners: [char; 4],
    title: Option<String>,
}

impl FrameBorder {
    pub fn new(
        top: u32,
        left: u32,
        width: u32,
        height: u32,
        corners: [char; 4],
        title: Option<String>,
    ) -> Result<FrameBorder, String> {
        if width < 3 || height < 3 {
            Err(String::from(
                "Border must have minimum width and height of 3 tiles to acommodate all elements",
            ))
        } else {
            Ok(FrameBorder {
                top,
                left,
                width,
                height,
                corners,
                title,
            })
        }
    }

    pub fn draw(&self, canvas: &mut Offscreen) {
        let left = self.left as i32;
        let top = self.top as i32;
        let width = self.width as i32;
        let height = self.height as i32;

        // draw corners
        canvas.set_char(left, top, self.corners[0]);
        canvas.set_char(left + width, top, self.corners[1]);
        canvas.set_char(left + width, top + height, self.corners[2]);
        canvas.set_char(left, top + height, self.corners[3]);

        // draw lines
        canvas.horizontal_line(left + 1, top, width - 1, BackgroundFlag::Alph);
        canvas.horizontal_line(left + 1, top + height, width - 1, BackgroundFlag::Alph);
        canvas.vertical_line(left, top + 1, height - 1, BackgroundFlag::Alph);
        canvas.vertical_line(left + width, top + 1, height - 1, BackgroundFlag::Alph);

        // Draw title if necessary
        if let Some(title) = &self.title {
            canvas.print((self.left + 1) as i32, self.top as i32, title);
        }
    }
}
