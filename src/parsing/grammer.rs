use crate::parsing::version_parser::{Version, Range, Op};

peg::parser!( pub grammar the_parser() for str {
  pub rule parse_version() -> Version
    = " "* v:version() " "* ![_] {v} // ![_] means end of file

  rule version() -> Version
    = ['v' | 'V']? " "? m:main() e:extra()? a:afterV() {
      Version::new_w_extra(
        m.0,
        m.1.unwrap_or(0),
        m.2.unwrap_or(0),
        e,
        a.0,
        a.1
      )
  }
  // pre and build any order and existence
  rule afterV() -> (Option<String>, Option<String>)
    // here end of file is kinda needed because if not it will accept afterV if the order is b p, cause "+window-alpha" will return (None, Some("window")) and come back without checking further
    = p:pre()? b:build()? ![_] { (p, b) }
    / b:build() p:pre() ![_] { (Some(p), Some(b)) }
  rule num() -> u32
    = n:$(['0'..='9']+) {? n.parse().or(Err("number")) } //n tenho a certeza do q {? rust} faz https://docs.rs/peg/latest/peg/#combining

  rule chars() -> String
    = n:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.']+) {? Ok(n.to_string())}

  rule separator() -> ()
    = n:$([' ' | ',' | ';']*) {} //* means 0 or more, + means 1 or more

  rule main() -> (u32, Option<u32>, Option<u32>)
    = M:num() "."? m:num()? "."? p:num()? { (M, m, p) }

  rule extra() -> String
    = "." c:chars() { c }

  rule build() -> String
    = "+" c:chars() { c }

  rule pre() -> String
    = "-" c:chars() { c }

  pub rule parse_range() -> Range
    = " "* r:(range() ** separator()) " "* ![_] { Range::from_ver_vec(r) }

  rule range() -> (Op, Version)
    = o:op() " "* v:version() " "* { (o,v) }

  rule op() -> Op
    = o:$("==" / "!=" / "<=" / ">=" / "=" / "<" / ">" / "~" / "^" / " " / "") { Op::from_str(o).unwrap() }
      // => and =< will fail, but that's ok
});