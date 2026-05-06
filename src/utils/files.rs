use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use actix_web::{Error, HttpResponse, Responder};
use std::path::Path;

pub async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

//#[post("/upload-file")]
pub async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./tmp".to_string());
    for f in form.files {
        let file_name = f
            .file_name
            .as_deref()
            .and_then(|name| Path::new(name).file_name())
            .and_then(|name| name.to_str())
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("upload.bin");
        let path = Path::new(&upload_dir).join(file_name);
        log::info!("saving to {}", path.display());
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}
