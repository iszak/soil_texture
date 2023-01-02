use soil::SoilTexture;

fn main() {
    let texture = SoilTexture::new(0.0, 0.0, 1.0).unwrap();

    match texture {
        SoilTexture::Clay { .. } => println!("clay"),
        _ => println!("not clay"),
    }
}
