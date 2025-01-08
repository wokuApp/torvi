use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        auth::controller::login,
        // Tournament endpoints
        tournaments::controller::create_tournament,
        tournaments::controller::get_tournament,
        tournaments::controller::list_tournaments,
        // Match endpoints
        matches::controller::create_match,
        matches::controller::get_match,
        // etc...
    ),
    components(
        schemas(
            auth::model::LoginRequest,
            auth::model::RegisterRequest,
            auth::model::AuthResponse,
            tournaments::model::Tournament,
            tournaments::model::CreateTournamentRequest,
            matches::model::Match,
            matches::model::CreateMatchRequest,
            error::Error
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "tournaments", description = "Tournament management endpoints"),
        (name = "matches", description = "Match management endpoints")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    fn configure_security() -> SecurityScheme {
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .description(Some("JWT token obtained from login endpoint"))
            .build()
    }
}
