fn main() {
    use presentation::{App, AppProps};

    use dioxus::prelude::*;
    use std::io::Write;

    // distのパス
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_path = std::path::Path::new(manifest_dir).join("../../dist_ssr");
    assert!(dist_path.exists());

    let index_html_text = std::fs::read_to_string(dist_path.join("index.html")).unwrap();

    let (base_html, _) = index_html_text.split_once("<body>").unwrap();

    let mut vdom = VirtualDom::new_with_props(
        App,
        AppProps {
            admin_password: "some password".to_string(),
        },
    );
    let _ = vdom.rebuild();

    let html_content = format!(
        r#"
{}
    <body>
        <div id="main">
{}
        </div>
    </body>
</html>
        "#,
        base_html,
        dioxus_ssr::pre_render(&vdom)
    );

    let mut file = std::fs::File::create("rendered.html").unwrap();
    file.write_all(html_content.as_bytes()).unwrap();
}
