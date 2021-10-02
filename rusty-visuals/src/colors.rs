use nannou::color::{self, Hsl};
use nannou::prelude::*;

const DEFAULT_SATURATION: color::DefaultScalar = 0.5;
const DEFAULT_LIGHTNESS: color::DefaultScalar = 0.5;

// Will pick a hue using a uniform probability distribution between hue_min and hue_max.
// Saturation and Lightness in the HSL color palette is fixed to constants.
pub fn random_color_in_hue_range(
    hue_min: color::DefaultScalar,
    hue_max: color::DefaultScalar,
) -> Hsl {
    let hue = map_range(random_range(0.0, 1.0), 0.0, 1.0, hue_min, hue_max);
    return Hsl::new(hue, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
}

pub fn random_color_in_range(
    hue_min: color::DefaultScalar,
    hue_max: color::DefaultScalar,
    saturation_min: color::DefaultScalar,
    saturation_max: color::DefaultScalar,
) -> Hsl {
    let hue = map_range(random_range(0.0, 1.0), 0.0, 1.0, hue_min, hue_max);
    let saturation = map_range(
        random_range(0.0, 1.0),
        0.0,
        1.0,
        saturation_min,
        saturation_max,
    );
    return Hsl::new(hue, saturation, DEFAULT_LIGHTNESS);
}
