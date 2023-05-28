// A Rusty thermometer

fn main() {
    let boiling_water_f: f64 = 212.0;
    let frozen_water_c: f64 = 0.0;

    let boiling_water_c = (boiling_water_f - 32.0) * (5.0 / 9.0);
    let frozen_water_f = (frozen_water_c * (9.0 / 5.0)) + 32.0;

    println!(
        "Water starts boiling at {}°C (or {}°F).",
        boiling_water_c, boiling_water_f
    );
    println!(
        "Water starts freezing at {}°C (or {}°F).",
        frozen_water_c, frozen_water_f
    );
}