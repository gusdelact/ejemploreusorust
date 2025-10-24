use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug, Default)]
struct Character {
    #[serde(default)]
    id: Option<u32>,
    #[serde(default)]
    age: Option<u32>,
    #[serde(default)]
    birthdate: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    gender: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    occupation: Option<String>,
    #[serde(default)]
    phrases: Option<Vec<String>>,
    #[serde(default)]
    portrait_path: Option<String>,
    #[serde(default)]
    status: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // URL del endpoint (puedes reemplazar por cualquier ID o ruta)
    let url = "https://api.sampleapis.com/simpsons/character/1";

    // Llamada HTTP
    let response = ureq::get(url).call()?;

    // Deserialización tolerante
    let character: Character = response.into_json()?;

    // --- Impresión segura ---
    println!("👤 Nombre: {}", character.name.clone().unwrap_or("Desconocido".to_string()));
    println!("🎂 Edad: {}", character.age.unwrap_or(0));
    println!("🧠 Ocupación: {}", character.occupation.clone().unwrap_or("Desconocida".to_string()));
    println!("📅 Nacimiento: {}", character.birthdate.clone().unwrap_or("N/A".to_string()));
    println!("📖 Descripción:\n{}\n", character.description.clone().unwrap_or("Sin descripción".to_string()));

    // Frases
    println!("💬 Frases:");
    if let Some(phrases) = character.phrases {
        for phrase in phrases {
            println!("  - {}", phrase);
        }
    } else {
        println!("  (no hay frases registradas)");
    }

    println!("\n📸 Imagen: {}", character.portrait_path.clone().unwrap_or("N/A".to_string()));
    println!("🩻 Estado: {}", character.status.clone().unwrap_or("Desconocido".to_string()));

    Ok(())
}
