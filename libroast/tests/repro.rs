use sha3::{
    Digest,
    Keccak256,
};
use std::{
    fs,
    fs::read,
    io,
    path::{
        Path,
        PathBuf,
    },
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};
use tracing_test::traced_test;
use walkdir::WalkDir;

fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error>
{
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        trace!(?entry);
        trace!(?ty);
        if ty.is_dir()
        {
            trace!(?ty, "Is directory?");
            copy_dir_all(entry.path(), &dst.join(entry.file_name()))?;

        // Should we respect symlinks?
        // } else if ty.is_symlink() {
        //     debug!("Is symlink");
        //     let path = fs::read_link(&entry.path())?;
        //     let path = fs::canonicalize(&path).unwrap();
        //     debug!(?path);
        //     let pathfilename = path.file_name().unwrap_or(OsStr::new("."));
        //     if path.is_dir() {
        //         copy_dir_all(&path, &dst.join(pathfilename))?;
        //     } else {
        //         fs::copy(&path, &mut dst.join(pathfilename))?;
        //     }

        // Be pedantic or you get symlink error
        }
        else if ty.is_file()
        {
            trace!(?ty, "Is file?");
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        };
    }
    Ok(())
}

const MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR", "No such manifest dir");

fn generate_gz_tarball(outpath: &Path) -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    copy_dir_all(src, workdir)?;
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();
    libroast::compress::targz(outpath, workdir, &updated_paths, true)?;
    let res = libroast::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_xz_tarball(outpath: &Path) -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    copy_dir_all(src, workdir)?;
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();
    libroast::compress::tarxz(outpath, workdir, &updated_paths, true)?;
    let res = libroast::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_zst_tarball(outpath: &Path) -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    copy_dir_all(src, workdir)?;
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();
    libroast::compress::tarzst(outpath, workdir, &updated_paths, true)?;
    let res = libroast::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_bz2_tarball(outpath: &Path) -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    copy_dir_all(src, workdir)?;
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();
    libroast::compress::tarbz2(outpath, workdir, &updated_paths, true)?;
    let res = libroast::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_icecream_tarball(outpath: &Path) -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    copy_dir_all(src, workdir)?;
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();
    libroast::compress::vanilla(outpath, workdir, &updated_paths, true)?;
    let res = libroast::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}
#[traced_test]
#[test]
fn repro_gz_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsofDeezNutsRepro1");
    generate_gz_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsofDeezNutsRepro2");
    generate_gz_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[traced_test]
#[test]
fn repro_xz_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsofJiaTanRepro1");
    generate_xz_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsofJiaTanRepro2");
    generate_xz_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[traced_test]
#[test]
fn repro_zst_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsfacebookRepro1");
    generate_zst_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsfacebookRepro2");
    generate_zst_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[traced_test]
#[test]
fn repro_bz2_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballswhatsbzRepro1");
    generate_bz2_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballswhatsbzRepro2");
    generate_bz2_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[traced_test]
#[test]
fn repro_vanilla_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsvanillacreampie");
    generate_icecream_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsvanillacreampie");
    generate_icecream_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}