use gleam_core::{Error, Result};

use crate::build;

pub fn command() -> Result<()> {
    let _config = crate::config::root_config()?;

    // Build the project to check that it is valid
    let _ = build::main()?;

    // TODO: Get hex username from user

    // TODO: Get hex password from user

    // TODO: Create API token

    // TODO: Build HTML documentation

    // TODO: Build HTML documentation

    // TODO: Read project files

    tracing::info!("Creating release tarball");
    let _tarball = build_tarball()?;

    // TODO: Publish release to hexpm

    // TODO: Publish docs to hexpm for release

    // TODO: Delete API token

    Ok(())
}

fn build_tarball() -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    {
        let mut tarball = tar::Builder::new(&mut bytes);
        tarball
            .append_dir_all("src", "src")
            .map_err(|e| Error::add_tar("src", e))?;
        tarball
            .append_path("gleam.toml")
            .map_err(|e| Error::add_tar("gleam.toml", e))?;
        tarball.finish().unwrap();
    }
    Ok(bytes)
}