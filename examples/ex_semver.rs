extern crate semver;

use semver::Version;
use semver::VersionReq;

fn main() {
    // parse
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: vec!(),
        build: vec!(),
    }));

    // comparison
    assert!(Version::parse("1.2.3-alpha") > Version::parse("1.2.0"));

    // increment_patch
    let mut bugfix_release = Version::parse("1.0.0").unwrap();
    bugfix_release.increment_patch();
    assert_eq!(Ok(bugfix_release), Version::parse("1.0.1"));

    // increment_minor
    let mut feature_release = Version::parse("1.4.6").unwrap();
    feature_release.increment_minor();
    assert_eq!(Ok(feature_release), Version::parse("1.5.0"));

    // increment_major
    let mut chrome_release = Version::parse("41.5.5377").unwrap();
    chrome_release.increment_major();
    assert_eq!(Ok(chrome_release), Version::parse("42.0.0"));

    // requirements
    let r = VersionReq::parse(">= 1.2.3").unwrap();
    assert!(r.to_string() > "1.2.0".to_string());
    let r = VersionReq::parse("~1.2.3").unwrap();
    let v = Version::parse("1.2.6").unwrap();
    assert!(r.matches(&v));
}
