pub fn generate_interceptor_js(pattern: &str) -> String {
    format!(
        r#"
        (() => {{
            window.__opencli_intercepted = window.__opencli_intercepted || [];
            window.__opencli_intercepted_errors = window.__opencli_intercepted_errors || [];
            const __pattern = {};

            if (!window.__opencli_interceptor_patched) {{
                const __checkMatch = (url) => __pattern && url.includes(__pattern);

                const __origFetch = window.fetch;
                window.fetch = async function(...args) {{
                    const reqUrl = typeof args[0] === 'string' ? args[0]
                        : (args[0] && args[0].url) || '';
                    const response = await __origFetch.apply(this, args);
                    if (__checkMatch(reqUrl)) {{
                        try {{
                            const clone = response.clone();
                            const json = await clone.json();
                            window.__opencli_intercepted.push(json);
                        }} catch(e) {{ window.__opencli_intercepted_errors.push({{ url: reqUrl, error: String(e) }}); }}
                    }}
                    return response;
                }};

                const __XHR = XMLHttpRequest.prototype;
                const __origOpen = __XHR.open;
                const __origSend = __XHR.send;
                __XHR.open = function(method, url) {{
                    this.__opencli_url = String(url);
                    return __origOpen.apply(this, arguments);
                }};
                __XHR.send = function() {{
                    if (__checkMatch(this.__opencli_url)) {{
                        this.addEventListener('load', function() {{
                            try {{
                                window.__opencli_intercepted.push(JSON.parse(this.responseText));
                            }} catch(e) {{ window.__opencli_intercepted_errors.push({{ url: this.__opencli_url, error: String(e) }}); }}
                        }});
                    }}
                    return __origSend.apply(this, arguments);
                }};

                window.__opencli_interceptor_patched = true;
            }}
        }})()
        "#,
        pattern
    )
}

pub fn generate_read_intercepted_js(array_name: &str) -> String {
    format!(
        r#"
        (() => {{
            const data = window.{} || [];
            window.{} = [];
            return data;
        }})()
        "#,
        array_name, array_name
    )
}
