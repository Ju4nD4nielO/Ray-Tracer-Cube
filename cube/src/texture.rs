use crate::color::Color;
use image::RgbaImage;

pub struct Texture {
    image: RgbaImage,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let image = image::open(path)
            .unwrap_or_else(|error| panic!("No se pudo cargar la textura '{path}': {error}"))
            .to_rgba8();

        Texture { image }
    }

    /// Muestrea el color de la textura en coordenadas UV normalizadas [0, 1].
    /// Los valores fuera de rango se envuelven (wrap), útil si la textura se
    /// repite en una cara del cubo.
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let (width, height) = self.image.dimensions();

        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);

        let x = ((u * width as f32) as u32).min(width - 1);
        let y = (((1.0 - v) * height as f32) as u32).min(height - 1);

        let pixel = self.image.get_pixel(x, y);

        Color::new(pixel[0], pixel[1], pixel[2])
    }
}