use std::sync::Arc;
use wasm_bindgen::prelude::*;


use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    service::handler::{
        get_me_handler, health_checker_handler, login_user_handler, logout_handler,
        register_user_handler,
    },
    authentication::jwt_auth::auth,
    AppState,
};

#[wasm_bindgen]
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/users/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}
