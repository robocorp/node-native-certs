use neon::prelude::*;
use x509_certificate::certificate::*;

fn load_native_certs(mut cx: FunctionContext) -> JsResult<JsString> {
    let result: Result<Vec<String>, _> = rustls_native_certs::load_native_certs()
        .map(|certs| {
            certs
                .into_iter()
                .map(|cert| {
                    let der = X509Certificate::from_der(&cert).expect("Could not import to DER");
                    let subject = der.subject_common_name().unwrap_or_else(|| String::new());
                    let issuer = der.issuer_common_name().unwrap_or_else(|| String::new());
                    let pem = der.encode_pem().expect("could not encode to PEM");
                    format!("# Subject: {subject}\n# Issuer: {issuer}\n{pem}\n")
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
