use serde::Serialize;
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;

use actix_web::{ post, Error as ActixError, HttpResponse };

#[derive(Serialize)]
struct Stats {
    lines: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    word_count: Option<usize>
}

#[post("/upload_stats")]
async fn upload_stats(
    mut payload: Multipart,
) -> Result<HttpResponse, ActixError> {
    let mut file_data = Vec::<u8>::new();
    let mut layout: Option<String> = Some("simple".to_owned());
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap();
        match field_name {
            "file" => {
                while let Some(chunk) = field.try_next().await? {
                    file_data.extend_from_slice(&chunk);
                }
            }
            "layout" => {
                let bytes = field.try_next().await?;
                layout = String::from_utf8(bytes.unwrap().to_vec()).ok();
            }
            _ => {}
        }
    }
    let file_content = std::str::from_utf8(&file_data)?;
    let mut i = 0;
    let mut word_count=0;
    for line in file_content.lines() {
        word_count+=line.chars().count();
        i += 1;
    }
    let word_count_res = if layout.unwrap() == String::from("advanced") {
        Some(word_count)
    } else {
        None
    };
    Ok(HttpResponse::Ok().json(Stats {
        lines: i,
        word_count: word_count_res
    }))
}