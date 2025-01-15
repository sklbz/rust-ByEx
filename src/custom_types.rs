#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
    weight: u16,
    height: u16,
    diet: String,
    species: String,
}

impl Animal {
    fn new(
        name: String,
        age: u8,
        weight: u16,
        height: u16,
        diet: String,
        species: String,
    ) -> Animal {
        Animal {
            name,
            age,
            weight,
            height,
            diet,
            species,
        }
    }
}

#[derive(Debug)]
struct Horse {
    animal: Animal,
}

impl Horse {
    fn new(name: String, age: u8, weight: u16, height: u16) -> Horse {
        Horse {
            animal: Animal {
                name,
                age,
                weight,
                height,
                diet: "🍎".to_string(),
                species: "🐎".to_string(),
            },
        }
    }
}
