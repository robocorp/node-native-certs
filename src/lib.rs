use neon::prelude::*;
use x509_certificate::certificate::*;

fn load_native_certs(mut cx: FunctionContext) -> JsResult<JsString> {
    let result: Result<Vec<String>, _> = rustls_native_certs::load_native_certs()
        .map(|certs| {
            certs
                .into_iter()
                .map(|cert| {
                    X509Certificate::from_der(&cert.0)
                    .expect("Could not import to DER")
                    .encode_pem()
                    .expect("could not encode to PEM")
                })
                .collect()
        });

    match result {
        Ok(certs) => {
            let concatenated_string = certs.join("");
            let js_string = cx.string(&concatenated_string);
            Ok(js_string)
        }
        Err(_) => cx.throw_error("Failed to load native certificates"),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("load_native_certs", load_native_certs)?;
    Ok(())
}
