#[macro_use] extern crate lalrpop_util;

use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use thiserror::Error;

lalrpop_mod!(pub version_parser);

#[derive(Error, Debug)]
enum ParseError {
  #[error("error in parsing version")]
  Version,
  #[error("error in parsing range")]
  Range,
}

pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  extra_version: Option<String>,
  pre_release: Option<String>,
  build: Option<String>,
}

impl Version {
  //noinspection RsUnresolvedReference
  fn parse(version: &str) -> Result<Self, ParseError> {
    let version: Self = version_parser::versionParser::new().parse(version).unwrap_or_else(|_| return Err(ParseError::Version));
    Ok(version)
  }
  fn new(
    major: u32,
    minor: u32,
    patch: u32,
    //1.1.0.1.5 < 1.1.0.1.6, 1.1.0.1.5 > 1.1.0, 1.1.0.0.0 > 1.1.0
    extra_version: Option<String>,
    // 1.1.0-rc.1 < 1.1.0-rc.2, 1-a < 1-b, 1.1.0-rc.1 <= 1.1.0
    /// # Pre-release-note
    /// é menor que ele mas no range é igual, tipo uma espécie de epsilon
    /// isto é porque o range espera-se que por exemplo >= 1.0, < 2.0 não inclua 2.0-alpha
    /// embora tecnicamente inclui pq é antes
    /// ainda assim quando for para comparar versões, 2.0-alpha é menor que 2.0 na mesma (por exemplo pra atualizar)
    pre_release: Option<String>,
    //1.1.0+build.1 = 1.1.0+build.2, 1.1.0+build.1 = 1.1.0
    build: Option<String>
  ) -> Self {
    Self {
      major,
      minor,
      patch,
      extra_version,
      pre_release,
      build,
    }
  }
  //could be a cool macro
  //maybe remove if not used anywhere
  fn with_major(&self, major: u32) -> Self {
    Self {
      major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_minor(&self, minor: u32) -> Self {
    Self {
      major: self.major,
      minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_patch(&self, patch: u32) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_extra_version(&self, extra_version: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version,
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_pre_release(&self, pre_release: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release,
      build: self.build.clone(),
    }
  }
  fn with_build(&self, build: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build,
    }
  }

  fn is(&self, other: &Self) -> bool {
    // comparasion with everything, and not equivelant
    self.major == other.major
      && self.minor == other.minor
      && self.patch == other.patch
      && self.extra_version == other.extra_version
      && self.pre_release == other.pre_release
      && self.build == other.build
  }
  fn is_older_than(&self, other: &Self) -> bool {
    // comparasion with everything, different than <= since pre_release is checked
    // check version_parser.rs#Pre-release-note
    self.major < other.major
      || self.minor < other.minor
      || self.patch < other.patch
      || self.extra_version < other.extra_version
      || self.pre_release < other.pre_release
  }
}

impl FromStr for Version {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl PartialEq for Version {
  fn eq(&self, other: &Self) -> bool {
    self.major == other.major
      && self.minor == other.minor
      && self.patch == other.patch
      && self.extra_version == other.extra_version
      && self.pre_release == other.pre_release
  }
}

impl Eq for Version {}

impl PartialOrd<Version> for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.major < other.major {
      Some(Ordering::Less)
    } else if self.major > other.major {
      Some(Ordering::Greater)
    } else if self.minor < other.minor {
      Some(Ordering::Less)
    } else if self.minor > other.minor {
      Some(Ordering::Greater)
    } else if self.patch < other.patch {
      Some(Ordering::Less)
    } else if self.patch > other.patch {
      Some(Ordering::Greater)
    } else if self.extra_version < other.extra_version { //TODO check if this works since it's optional
      Some(Ordering::Less)
    } else if self.extra_version > other.extra_version {
      Some(Ordering::Greater)
      // pre-release isn't checked because this is for implmenting ranges, see version_parser.rs#Pre-release-note
    } else {
      Some(Ordering::Equal)
    }
  }
}
impl Ord for Version {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl ToString for Version {
  fn to_string(&self) -> String {
    let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
    if let Some(extra_version) = &self.extra_version {
      s.push_str(&format!(".{}", extra_version));
    }
    if let Some(pre_release) = &self.pre_release {
      s.push_str(&format!("-{}", pre_release));
    }
    if let Some(build) = &self.build {
      s.push_str(&format!("+{}", build));
    }
    s
  }
}


struct Range {
  min: Option<Version>, //inclusive
  max: Option<Version>, //exclusive, because it's hard to go back to the previous version
  except: Vec<Version>,
  include: Vec<Version>,
}

impl ToString for Range {
  fn to_string(&self) -> String {
    if self.is_any() {
      return "*".to_string();
    }
    let mut s = String::new();
    if let Some(min) = &self.min {
      s.push_str(&format!(">={},", min.to_string()));
    }
    if let Some(max) = &self.max {
      s.push_str(&format!("<{},", max.to_string()));
    }
    for except in &self.except {
      s.push_str(&format!("!={},", except.to_string()));
    }
    for include in &self.include {
      s.push_str(&format!("={},", include.to_string()));
    }
    s.pop(); //remove the last comma
    s
  }
}

impl Range {
  fn contains(&self, version: Version) -> bool {
    todo!()
  }
  fn is_any(&self) -> bool { // is empty or is just >= 0.0.0
    todo!()
  }
  fn is_valid(&self) -> bool { // is not empty and min <= max and is not < 0.0.0
    todo!()
  }
  fn is_exact_match(&self) -> bool { // min == max or just includes one version
    todo!()
  }
  fn from_ver_vec(ranges: Vec<(Op, Version)>) -> Self {
    // Sort the ranges by version number
    let mut ranges = Self::sort_vec(ranges);
    // get lowest ge, and highest lt (since it's sorted)
    let min = ranges.first().map(|(_, v)| *v);
    let max = ranges.last().map(|(_, v)| *v);
    // get all the except and include (Ne and Eq)
    let except: Vec<_> = ranges
      .into_iter()
      .filter(|(op, _)| *op == Op::Ne)
      .map(|(_, v)| v)
      .collect();
    let include: Vec<_> = ranges
      .into_iter()
      .filter(|(op, _)| *op == Op::Eq)
      .map(|(_, v)| v)
      .collect();
    Range { //Note: this can return an invalid range, that's why we have is_valid
      min,
      max,
      except,
      include,
    }
  }
  fn mixed_vec_to_stand_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Expand tilde, caret, le and gt ranges to simple lt and ge ranges
    ranges.into_iter().flat_map(|(op, version)| {
      match op {
        Op::Tilde => Self::tilde_range_to_vec(version),
        Op::Caret => Self::caret_range_to_vec(version),
        Op::Le => Self::le_range_to_vec(version),
        Op::Gt => Self::gt_range_to_vec(version),
        _ => vec![(op, version)],
      }
    }).collect::<Vec<_>>()
  }

  fn sort_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Sort the ranges by version number

    let mut ranges = Self.mixed_vec_to_stand_vec(ranges);
    ranges.sort_by(|(_, a), (_, b)| a.cmp(&b));
    ranges
  }

  //noinspection RsUnresolvedReference
  fn parse(range: &str) -> Result<Self, ParseError> {
    let range: Vec<(Op, Version)>  = version_parser::rangesParser::new().parse(range).unwrap_or_else(|_| return Err(ParseError::Range));
    Ok(Self::from_ver_vec(range))
  }

  fn tilde_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ~1.2.3 -> >=1.2.3 <1.3.0
    // ~1.2 -> >=1.2.0 <1.3.0
    // ~1 -> >=1.0.0 <1.1.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major, version.minor + 1, 0, None, None, None)),
    ]
  }
  fn caret_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ^1.2.3 -> >=1.2.3 <2.0.0
    // ^1.2 -> >=1.2.0 <2.0.0
    // ^1 -> >=1.0.0 <2.0.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major + 1, 0, 0, None, None, None)),
    ]
  }
  fn le_range_to_lt(version: Version) -> Vec<(Op, Version)> {
    // <=1.2.3 -> <1.2.4
    // <=1.2 -> <1.2.1
    // <=1 -> <1.0.1
    vec![
      (Op::Lt, Version::new(version.major, version.minor, version.patch + 1, None, None, None)),
    ]
  }
  fn gt_range_to_ge(version: Version) -> Vec<(Op, Version)> {
    // >1.2.3 -> >=1.2.4
    // >1.2 -> >=1.2.1
    // >1 -> >=1.0.1
    vec![
      (Op::Ge, Version::new(version.major, version.minor, version.patch + 1, None, None, None)),
    ]
  }

}

pub enum Op {
  Eq,    //==
  Ne,    //!=
  Gt,    //>
  Lt,    //<
  Ge,    //>=
  Le,    //<=
  Tilde, //~
  Caret  //^
}

impl Op {
  fn from_str(op: &str) -> Result<Self, ParseError> {
    match op {
      "==" | "=" | "" => Ok(Self::Eq),
      "!=" => Ok(Self::Ne),
      ">" => Ok(Self::Gt),
      "<" => Ok(Self::Lt),
      ">=" => Ok(Self::Ge),
      "<=" => Ok(Self::Le),
      "~" => Ok(Self::Tilde),
      "^" => Ok(Self::Caret),
      _ => Err(ParseError::Range)
    }
  }
}