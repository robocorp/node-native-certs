use pki_types::CertificateDer;

use std::io::Error;

pub fn load_native_certs() -> Result<Vec<CertificateDer<'static>>, Error> {
    // My                 "Personal"
    // Root               "Trusted Root Certification Authorities"
    // Trust              "Enterprise Trust"
    // CA                 "Intermediate Certification Authorities"
    let store_names = ["My", "Root", "Trust", "CA"];
    let mut certs = Vec::new();

    for &store_name in &store_names {
        // Try to open the store from the current user context and accumulate certificates
        if let Ok(current_user_store) = schannel::cert_store::CertStore::open_current_user(store_name) {
            for cert in current_user_store.certs() {
                certs.push(CertificateDer::from(cert.to_der().to_vec()));
            }
        }

        // Try to open the store from the local machine context and accumulate certificates
        if let Ok(local_machine_store) = schannel::cert_store::CertStore::open_local_machine(store_name) {
            for cert in local_machine_store.certs() {
                certs.push(CertificateDer::from(cert.to_der().to_vec()));
            }
        }
    }

    Ok(certs)
}