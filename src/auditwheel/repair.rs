use super::audit::AuditWheelError;
use crate::auditwheel::Policy;
use anyhow::Result;
use lddtree::DependencyAnalyzer;
use std::path::{Path, PathBuf};

pub fn find_external_libs(
    artifact: impl AsRef<Path>,
    policy: &Policy,
    sysroot: PathBuf,
) -> Result<Vec<lddtree::Library>, AuditWheelError> {
    let dep_analyzer = DependencyAnalyzer::new(sysroot);
    let deps = dep_analyzer
        .analyze(artifact)
        .map_err(AuditWheelError::DependencyAnalysisError)?;
    let mut ext_libs = Vec::new();
    for (name, lib) in deps.libraries {
        // Skip dynamic linker/loader and white-listed libs
        if name.starts_with("ld-linux")
            || name == "ld64.so.2"
            || name == "ld64.so.1"
            // musl libc, eg: libc.musl-aarch64.so.1
            || name.starts_with("libc.")
            || policy.lib_whitelist.contains(&name)
        {
            continue;
        }
        ext_libs.push(lib);
    }
    Ok(ext_libs)
}
