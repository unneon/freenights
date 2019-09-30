use amethyst::core::math::{Complex, Vector2};

pub fn scale_axes(x: f32, y: f32) -> Vector2<f32> {
	if x == 0. && y == 0. {
		return Vector2::zeros();
	}
	let (_, theta) = Complex::new(x, y).to_polar();
	let max_x = f32::min(1., 1. / theta.tan().abs());
	let max_y = f32::min(1., theta.tan().abs());
	let max_r = f32::hypot(max_x, max_y);
	Vector2::new(x, y) / max_r
}
