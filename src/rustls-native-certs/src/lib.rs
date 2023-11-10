//! `rustls-native-certs` allows `rustls` to use the platform's native certificate
//! store when operating as a TLS client.
//!
//! It provides a single function [`load_native_certs()`], which returns a
//! collection of certificates found by reading the platform-native
//! certificate store.
//!
//! [`CertificateDer`] here is a marker newtype that denotes a DER-encoded
//! X.509 certificate encoded as a `Vec<u8>`.
//!
//! If you want to load these certificates into a `rustls::RootCertStore`,
//! you'll likely want to do something like this:
//!
//! ```no_run
//! let mut roots = rustls::RootCertStore::empty();
//! for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
//!     roots.add(cert).unwrap();
//! }
//! ```

#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
use unix as platform;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as platform;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

use std::io::{Error};

use pki_types::CertificateDer;

/// Load root certificates found in the platform's native certificate store.
///
/// This function fails in a platform-specific way, expressed in a `std::io::Error`.
///
/// This function can be expensive: on some platforms it involves loading
/// and parsing a ~300KB disk file. It's therefore prudent to call
/// this sparingly.
pub fn load_native_certs() -> Result<Vec<CertificateDer<'static>>, Error> {
    platform::load_native_certs()
}

/// Used inside unix.rs
fn load_pem_certs(path: &Path) -> Result<Vec<CertificateDer<'static>>, Error> {
    let f = File::open(path)?;
    let mut f = BufReader::new(f);
    rustls_pemfile::certs(&mut f)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("could not load PEM file {path:?}: {err}"),
            )
        })
}