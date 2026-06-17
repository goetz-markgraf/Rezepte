use crate::config::Config;
use crate::error::AppError;
use image::GenericImageView;
use image::ImageFormat;
use serde::{Deserialize, Serialize};

const MAX_IMAGE_BYTES: usize = 3500 * 1024;
const MAX_DIMENSION: u32 = 2048;

pub struct RecipeExtract {
    pub title: String,
    pub ingredients: String,
    pub instructions: String,
    pub category: Option<String>,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: Vec<ContentPart>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: String,
}

#[derive(Deserialize)]
struct ExtractedRecipe {
    title: String,
    ingredients: String,
    instructions: String,
    category: Option<String>,
}

const SYSTEM_PROMPT: &str = r####"Du bist ein Assistent, der Rezepte aus Fotos extrahiert.
Analysiere das Bild und extrahiere das Rezept.
Antworte ausschließlich mit einem JSON-Objekt (kein Markdown, kein Code-Block um das JSON herum) mit diesen Feldern:
- title: Name des Rezepts (String)
- ingredients: Zutaten als Text (String, Zeilenumbrüche mit \n)
- instructions: Zubereitungsanleitung als Text (String, Zeilenumbrüche mit \n)
- category: Eine der folgenden Kategorien oder null wenn keine passt: "Mittagessen", "Brot", "Party", "Kuchen", "Snacks"

Für ingredients und instructions sollst du direkt einfaches Markdown erzeugen, damit der Text in der App gut lesbar ist.
Bevorzugte Formatierung:
- Zutaten möglichst als Aufzählung mit `- `
- Zubereitung möglichst als nummerierte Liste mit `1.`
- Bei Unterabschnitten sparsam Überschriften mit `### `
- Wenn hilfreich: `**fett**`, `*kursiv*`, `` `inline code` ``, `~~durchgestrichen~~`
- Tabellen sind erlaubt, aber nur wenn sie im Foto klar erkennbar und wirklich hilfreich sind
- Aufgabenlisten `- [ ]` / `- [x]` nur verwenden, wenn sie im Bild tatsächlich so vorkommen

Wichtige Regeln:
- Kein Markdown um das JSON herum
- In den JSON-Strings Zeilenumbrüche als \n kodieren
- Inhalt nicht erfinden; fehlende oder unleserliche Stellen lieber neutral und knapp formulieren
- Offensichtliche OCR-Fehler still korrigieren

Beispiel:
{"title":"Apfelkuchen","ingredients":"### Teig\n- 200 g Mehl\n- 3 Äpfel","instructions":"1. Äpfel schälen.\n2. Teig verrühren.\n3. **Backen** bei 180 °C.","category":"Kuchen"}"####;

fn compress_for_vision(image_bytes: &[u8], mime_type: &str) -> Result<(Vec<u8>, String), AppError> {
    if image_bytes.len() <= MAX_IMAGE_BYTES {
        return Ok((image_bytes.to_vec(), mime_type.to_string()));
    }

    let format = match mime_type {
        "image/png" => ImageFormat::Png,
        "image/webp" => ImageFormat::WebP,
        _ => ImageFormat::Jpeg,
    };

    let mut img = image::load_from_memory_with_format(image_bytes, format)
        .or_else(|_| image::load_from_memory(image_bytes))
        .map_err(|e| AppError::BadRequest(format!("Bild konnte nicht dekodiert werden: {}", e)))?;

    let (w, h) = img.dimensions();
    if w > MAX_DIMENSION || h > MAX_DIMENSION {
        img = img.thumbnail(MAX_DIMENSION, MAX_DIMENSION);
    }

    let mut buf = std::io::Cursor::new(Vec::new());
    let mut quality = 80u8;

    loop {
        buf.set_position(0);
        buf.get_mut().clear();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
        img.write_with_encoder(encoder)
            .map_err(|e| AppError::BadRequest(format!("Bild konnte nicht kodiert werden: {}", e)))?;

        if buf.get_ref().len() <= MAX_IMAGE_BYTES || quality <= 50 {
            break;
        }
        quality -= 30;
    }

    tracing::info!(
        original_size = image_bytes.len(),
        compressed_size = buf.get_ref().len(),
        quality,
        "Bild fuer Vision-API komprimiert"
    );

    Ok((buf.into_inner(), "image/jpeg".to_string()))
}

pub async fn analyze_image(
    config: &Config,
    image_bytes: &[u8],
    mime_type: &str,
) -> Result<RecipeExtract, AppError> {
    let api_url = config
        .vision_api_url
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Vision-API nicht konfiguriert".to_string()))?;
    let api_key = config
        .vision_api_key
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Vision-API-Key nicht konfiguriert".to_string()))?;

    let (image_bytes, mime_type) = compress_for_vision(image_bytes, mime_type)?;
    let base64_image = base64_encode(&image_bytes);
    let data_url = format!("data:{};base64,{}", mime_type, base64_image);

    let request_body = ChatRequest {
        model: config.vision_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: vec![ContentPart::Text {
                    text: SYSTEM_PROMPT.to_string(),
                }],
            },
            Message {
                role: "user".to_string(),
                content: vec![
                    ContentPart::Text {
                        text: "Extrahiere das Rezept aus diesem Foto.".to_string(),
                    },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl { url: data_url },
                    },
                ],
            },
        ],
        response_format: ResponseFormat {
            format_type: "json_object".to_string(),
        },
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| AppError::BadRequest(format!("HTTP-Client-Fehler: {}", e)))?;

    let endpoint = format!("{}/chat/completions", api_url.trim_end_matches('/'));
    tracing::info!(endpoint = %endpoint, model = %config.vision_model, "Vision-API Anfrage");

    let response = client
        .post(&endpoint)
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(endpoint = %endpoint, error = %e, "Vision-API Verbindungsfehler");
            AppError::BadRequest(format!(
                "Foto konnte nicht analysiert werden. Bitte erneut versuchen oder das Rezept manuell eingeben. ({})",
                e
            ))
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "<kein Body>".to_string());
        tracing::error!(
            endpoint = %endpoint,
            status = %status,
            response_body = %body,
            "Vision-API Fehlerantwort"
        );
        return Err(AppError::BadRequest(format!(
            "Foto konnte nicht analysiert werden. Bitte erneut versuchen oder das Rezept manuell eingeben. (HTTP {})",
            status
        )));
    }

    let chat_response: ChatResponse = response.json().await.map_err(|e| {
        tracing::error!(error = %e, "Vision-API Antwort konnte nicht geparst werden");
        AppError::BadRequest(format!(
            "Antwort der Vision-API konnte nicht verarbeitet werden: {}",
            e
        ))
    })?;

    let content = chat_response
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or_else(|| {
            tracing::error!("Vision-API lieferte leere choices");
            AppError::BadRequest("Leere Antwort von der Vision-API".to_string())
        })?;

    tracing::debug!(content = %content, "Vision-API Rohinhalt empfangen");

    let extracted: ExtractedRecipe = serde_json::from_str(&content).map_err(|e| {
        tracing::error!(content = %content, error = %e, "JSON-Parsing der Vision-Antwort fehlgeschlagen");
        AppError::BadRequest(format!(
            "Rezept konnte nicht aus dem Foto extrahiert werden: {}",
            e
        ))
    })?;

    let valid_categories = ["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"];
    let category = extracted.category.filter(|c| valid_categories.contains(&c.as_str()));

    Ok(RecipeExtract {
        title: extracted.title,
        ingredients: extracted.ingredients,
        instructions: extracted.instructions,
        category,
    })
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let combined = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[((combined >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((combined >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(combined & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encode_hello() {
        assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
    }

    #[test]
    fn base64_encode_empty() {
        assert_eq!(base64_encode(b""), "");
    }

    #[test]
    fn base64_encode_abc() {
        assert_eq!(base64_encode(b"abc"), "YWJj");
    }

    #[test]
    fn invalid_category_filtered_out() {
        let valid = ["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"];
        let candidate = Some("Frühstück".to_string());
        let result = candidate.filter(|c| valid.contains(&c.as_str()));
        assert!(result.is_none());
    }

    #[test]
    fn valid_category_passes_through() {
        let valid = ["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"];
        let candidate = Some("Kuchen".to_string());
        let result = candidate.filter(|c| valid.contains(&c.as_str()));
        assert_eq!(result, Some("Kuchen".to_string()));
    }

    #[tokio::test]
    async fn analyze_image_fails_without_config() {
        let config = Config {
            database_url: "sqlite::memory:".to_string(),
            port: 8080,
            vision_api_url: None,
            vision_api_key: None,
            vision_model: "gpt-4o".to_string(),
        };
        let result = analyze_image(&config, b"fake", "image/jpeg").await;
        assert!(result.is_err());
    }

    #[test]
    fn system_prompt_erwaehnt_markdown_formatierung() {
        assert!(SYSTEM_PROMPT.contains("Markdown"));
        assert!(SYSTEM_PROMPT.contains("`- `"));
        assert!(SYSTEM_PROMPT.contains("`1.`"));
        assert!(SYSTEM_PROMPT.contains("**fett**"));
        assert!(SYSTEM_PROMPT.contains("### "));
    }

    #[test]
    fn compress_small_image_unchanged() {
        let img = image::RgbImage::from_pixel(100, 100, image::Rgb([128, 128, 128]));
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_with_encoder(image::codecs::jpeg::JpegEncoder::new(&mut buf))
            .unwrap();
        let jpeg_bytes = buf.into_inner();
        let result = compress_for_vision(&jpeg_bytes, "image/jpeg").unwrap();
        assert_eq!(result.0, jpeg_bytes);
        assert_eq!(result.1, "image/jpeg");
    }

    #[test]
    fn compress_large_image_reduces_size() {
        let mut img = image::RgbImage::new(2500, 1800);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = ((x * 17 + y * 31) % 256) as u8;
            let g = ((x * 7 + y * 13) % 256) as u8;
            let b = ((x * 23 + y * 41) % 256) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_with_encoder(image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 95))
            .unwrap();
        let jpeg_bytes = buf.into_inner();
        assert!(jpeg_bytes.len() > MAX_IMAGE_BYTES, "test image must exceed limit, got {} bytes", jpeg_bytes.len());

        let (compressed, mime) = compress_for_vision(&jpeg_bytes, "image/jpeg").unwrap();
        assert!(compressed.len() <= MAX_IMAGE_BYTES);
        assert_eq!(mime, "image/jpeg");
    }

    #[test]
    fn compress_thumbnail_applied_for_oversized_dimensions() {
        let mut img = image::RgbImage::new(3000, 2500);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = ((x * 17 + y * 31) % 256) as u8;
            let g = ((x * 7 + y * 13) % 256) as u8;
            let b = ((x * 23 + y * 41) % 256) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_with_encoder(image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 95))
            .unwrap();
        let jpeg_bytes = buf.into_inner();
        assert!(jpeg_bytes.len() > MAX_IMAGE_BYTES, "test image must exceed limit, got {} bytes", jpeg_bytes.len());

        let (compressed, _) = compress_for_vision(&jpeg_bytes, "image/jpeg").unwrap();
        let reloaded = image::load_from_memory(&compressed).unwrap();
        let (w, h) = reloaded.dimensions();
        assert!(w <= MAX_DIMENSION);
        assert!(h <= MAX_DIMENSION);
    }
}
