use indoc::formatdoc;

use crate::{config::DataSourceKind, state::AppState};


pub fn display_banner(state: &AppState) -> String {
    let version = &state.version;
    let rust_version = &state.rust_version;
    let commit = &state.commit;
    let data_source = &state.config.source_kind;
    let data_location = &state.config.source_location;
    let data_info = match data_source {
        DataSourceKind::Null => "Null".into(),
        _ => format!("{data_source:?}({data_location})"),
    };
    let issuers = state
        .config
        .jwt_issuers
        .as_ref()
        .map(|iss| {
            iss.iter()
                .map(|url| format!("\n - {url}"))
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_else(|| "no issuers accepted".into());
    let address = &state.config.address;
    let port = &state.config.port;
    formatdoc!(
        r#"
        --- TMS Resources Provider ---
        Version: {version}
        Commit: {commit}
        Rust version: {rust_version}

        Using Data source: {data_info}
        Issuers: {issuers}
        Listening at: {address}:{port}
    "#
    )
}