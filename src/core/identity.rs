use std::{env, fs, path::{Path, PathBuf}};
use libp2p::identity::Keypair;

pub fn default_identity_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = env::var_os("HOME").ok_or("HOME not set")?;
    let mut p = PathBuf::from(home);
    p.push(".peerboard/identity.key");
    Ok(p)
}

pub fn parse_identity_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--identity-file" {
            if let Some(v) = args.next() {
                return Ok(PathBuf::from(v));
            }
        }
        if let Some(v) = arg.strip_prefix("--identity-file=") {
            return Ok(PathBuf::from(v));
        }
    }

    default_identity_path()
}

pub fn load_or_generate_identity(path: &Path)
    -> Result<Keypair, Box<dyn std::error::Error>>
{
    if path.exists() {
        let bytes = fs::read(path)?;
        return Ok(Keypair::from_protobuf_encoding(&bytes)?);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let key = Keypair::generate_ed25519();
    fs::write(path, key.to_protobuf_encoding()?)?;
    Ok(key)
}