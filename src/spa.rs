use rocket::fs::NamedFile;
use std::path::PathBuf;

const SPA_DIR: &str = "web/landing/out";

/// Catch-all fallback: serves index.html for client-side routing.
/// Rank 20 ensures API, health, and WebSocket routes take priority.
#[get("/<_path..>", rank = 20)]
pub async fn fallback(_path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from(SPA_DIR).join("index.html"))
        .await
        .ok()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fallback]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spa_routes_count() {
        let routes = routes();
        assert_eq!(routes.len(), 1);
    }

    #[test]
    fn test_spa_dir_constant() {
        assert_eq!(SPA_DIR, "web/landing/out");
    }
}
