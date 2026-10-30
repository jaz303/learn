use tiny_skia::{Color, Paint, Pixmap, PixmapPaint, Rect, Transform};

pub struct DrawingContext {
    width: u32,
    height: u32,
    pub image: Pixmap,

    fill_color: Color,
}

impl DrawingContext {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let image = Pixmap::new(width, height);
        match image {
            Some(img) => Ok(Self {
                width,
                height,
                image: img,
                fill_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0).unwrap(),
            }),
            None => Err("failed to create image".into()),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let image = Pixmap::new(width, height);
        match image {
            Some(mut new_image) => {
                new_image.fill(Color::from_rgba(0., 0., 0., 1.).unwrap());
                let paint = PixmapPaint::default();
                new_image.draw_pixmap(
                    0,
                    0,
                    self.image.as_ref(),
                    &paint,
                    Transform::identity(),
                    None,
                );
                self.width = width;
                self.height = height;
                self.image = new_image;
                Ok(())
            }
            None => Err("failed to resize image".into()),
        }
    }

    pub fn set_fill(&mut self, color: tiny_skia::Color) {
        self.fill_color = color;
    }

    pub fn clear(&mut self) {
        self.image
            .fill(Color::from_rgba(0.0, 0.0, 0.0, 1.0).unwrap());
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let mut paint = Paint::default();
        paint.set_color(self.fill_color);

        self.image.fill_rect(
            Rect::from_xywh(x, y, width, height).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );
    }
}
