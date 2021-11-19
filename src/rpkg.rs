//
// rpkg-bundled v1.2.2
// made with <3 by c0repwn3r
//
// basically just rpkg with all the actual packaging
// stuff ripped out, it's only purpose is to source and
// install itself and rpl
//
use std::vec::Vec;

struct Package {
  pkg_name: String,
  pkg_url: String,
  pkg_version: String
}

struct RepoPackage {
  pkg: Package,
  fdwlurl: String
}

struct Repository {
  repo_name: String,
  repo_url: String,
  repo_rdl_hash: String,
  packages: Vec<RepoPackage>
}

fn source_bootstrapped(pkg: Package) {
  println!("rpkg: sourcing {}", pkg.pkg_name);
  println!("rpkg: initializing bootstrap repository");
  let rpl = Package {
    pkg_name: "rpl".to_string(),
    pkg_url: ".localpkg/.rpl-internal.rpb".to_string(),
    pkg_version: "%INSTALLVER%".to_string()
  };
  let rpkg = Package {
    pkg_name: "rpkg".to_string(),
    pkg_url: ".localpkg/.rpkg-internal.rpb".to_string(),
    pkg_version: "1.2.2".to_string()
  };
  let rrpl = RepoPackage {
    pkg: rpl,
    fdwlurl: "./lpdu".to_string()
  };
  let rrpkg = RepoPackage {
    pkg: rpkg,
    fdwlurl: "./lpdu".to_string()
  };
  let bootstraprepo = Repository {
    repo_name: "bootstrap_repository".to_string(),
    repo_url: "./lpdu".to_string(),
    repo_rdl_hash: "DB1C5FD517526B58E69021768BF73645F2E512714B087EAFA8E081FB9CE93165".to_string(), // sha256 of thisisanonsensevalue
    packages: vec![rrpl, rrpkg]
  };
  println!("Loaded base (bootstrap) repository");
}
