use crate::builders::PublishBuilder;
use crate::util::{RequestHelper, TestApp};
use crates_io_tarball::TarballBuilder;
use http::StatusCode;

#[test]
fn workspace_inheritance() {
    let (_app, _anon, _cookie, token) = TestApp::full().with_token();

    let tarball = TarballBuilder::new("foo", "1.0.0")
        .add_raw_manifest(b"[package]\nname = \"foo\"\nversion.workspace = true\n")
        .build();

    let response = token.publish_crate(PublishBuilder::new("foo", "1.0.0").tarball(tarball));
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.into_json(),
        json!({ "errors": [{ "detail": "failed to parse `Cargo.toml` manifest file\n\nvalue from workspace hasn't been set" }] })
    );
}

#[test]
fn workspace_inheritance_with_dep() {
    let (_app, _anon, _cookie, token) = TestApp::full().with_token();

    let tarball = TarballBuilder::new("foo", "1.0.0")
        .add_raw_manifest(
            b"[package]\nname = \"foo\"\nversion = \"1.0.0\"\n\n[dependencies]\nserde.workspace = true\n",
        )
        .build();

    let response = token.publish_crate(PublishBuilder::new("foo", "1.0.0").tarball(tarball));
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.into_json(),
        json!({ "errors": [{ "detail": "failed to parse `Cargo.toml` manifest file\n\nvalue from workspace hasn't been set" }] })
    );
}
