use pki_types::CertificateDer;

use std::io::Error;

static PKIX_SERVER_AUTH: &str = "1.3.6.1.5.5.7.3.1";

fn usable_for_rustls(uses: schannel::cert_context::ValidUses) -> bool {
    match uses {
        schannel::cert_context::ValidUses::All => true,
        schannel::cert_context::ValidUses::Oids(strs) => strs
            .iter()
            .any(|x| x == PKIX_SERVER_AUTH),
    }
}

pub fn load_native_certs() -> Result<Vec<CertificateDer<'static>>, Error> {
    let store_names = ["My", "Root", "Trust", "CA"];
    let mut certs = Vec::new();

    for &store_name in &store_names {
        // Try to open the store from the current user context and accumulate certificates
        if let Ok(current_user_store) = schannel::cert_store::CertStore::open_current_user(store_name) {
            for cert in current_user_store.certs() {
                if usable_for_rustls(cert.valid_uses().unwrap()) && cert.is_time_valid().unwrap() {
                    certs.push(CertificateDer::from(cert.to_der().to_vec()));
                }
            }
        }

        // Try to open the store from the local machine context and accumulate certificates
        if let Ok(local_machine_store) = schannel::cert_store::CertStore::open_local_machine(store_name) {
            for cert in local_machine_store.certs() {
                if usable_for_rustls(cert.valid_uses().unwrap()) && cert.is_time_valid().unwrap() {
                    certs.push(CertificateDer::from(cert.to_der().to_vec()));
                }
            }
        }
    }

    Ok(certs)
}