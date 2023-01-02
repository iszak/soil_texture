# Read me

This library provides an API to classify the soil texture based on the 
percentage of clay, sand, silt as defined in the USDA soil texture triangle 
calculator. This does not implement the classification of coarse, fine and
very fine sands or loams.

![USDA Soil Texture triangle chart](https://upload.wikimedia.org/wikipedia/commons/8/89/USDA_Soil_Texture.svg)

**This library is unlicensed and not published on Crates.io, please open an
issue if you want it published or licensed.**

```rust

use soil::SoilTexture;

match SoilTexture::new(0.0, 0.0, 1.0); {
    SoilTexture::Clay(_, _, _) => println!("clay"),
    _ => println!("not clay"),
    SoilTexture::Unknown => panic!("clay"),
}


```


*[USDA]: United States Department of Agriculture
