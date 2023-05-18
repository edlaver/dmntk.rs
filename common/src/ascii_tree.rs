use crate::ASCII_RESET;
use std::fmt;
use std::fmt::{Display, Formatter, Write};

pub struct AsciiText {
  color: Option<String>,
  text: String,
}

impl AsciiText {
  pub fn new(text: &str) -> Self {
    Self {
      color: None,
      text: text.to_string(),
    }
  }
  pub fn with_color(text: &str, color: &str) -> Self {
    Self {
      color: Some(color.to_string()),
      text: text.to_string(),
    }
  }
}

impl Display for AsciiText {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(color) = &self.color {
      write!(f, "{}{}{}", color, self.text, ASCII_RESET)
    } else {
      write!(f, "{}", self.text)
    }
  }
}

pub struct AsciiLine(Vec<AsciiText>);

impl AsciiLine {
  ///
  pub fn new(line: Vec<AsciiText>) -> Self {
    Self(line)
  }

  ///
  pub fn builder() -> AsciiLineBuilder {
    AsciiLineBuilder(vec![])
  }
}

impl Display for AsciiLine {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for text in &self.0 {
      write!(f, "{}", text)?
    }
    Ok(())
  }
}

pub struct AsciiLineBuilder(Vec<AsciiText>);

impl AsciiLineBuilder {
  ///
  pub fn text(mut self, text: &str) -> Self {
    self.0.push(AsciiText::new(text));
    self
  }

  ///
  pub fn with_color(mut self, text: &str, color: &str) -> Self {
    self.0.push(AsciiText::with_color(text, color));
    self
  }

  ///
  pub fn space(mut self) -> Self {
    self.0.push(AsciiText::new(" "));
    self
  }

  ///
  pub fn build(self) -> AsciiLine {
    AsciiLine(self.0)
  }
}

pub enum AsciiNode {
  Node(AsciiLine, Vec<AsciiNode>),
  Leaf(Vec<AsciiLine>),
}

impl AsciiNode {
  pub fn leaf_builder() -> AsciiLeafBuilder {
    AsciiLeafBuilder(vec![])
  }
}

pub fn write_ascii_tree(f: &mut dyn Write, node: &AsciiNode) -> fmt::Result {
  write_ascii_node(f, node, &vec![])
}

fn write_ascii_node(f: &mut dyn Write, tree: &AsciiNode, level: &Vec<usize>) -> fmt::Result {
  const NONE: &str = "   ";
  const EDGE: &str = " └─";
  const PIPE: &str = " │ ";
  const FORK: &str = " ├─";

  let max_pos = level.len();
  let mut second_line = String::new();
  for (pos, l) in level.iter().enumerate() {
    let last_row = pos == max_pos - 1;
    if *l == 1 {
      if !last_row {
        write!(f, "{}", NONE)?
      } else {
        write!(f, "{}", EDGE)?
      }
      second_line.push_str(NONE);
    } else {
      if !last_row {
        write!(f, "{}", PIPE)?
      } else {
        write!(f, "{}", FORK)?
      }
      second_line.push_str(PIPE);
    }
  }
  match tree {
    AsciiNode::Node(title, children) => {
      let mut deep = children.len();
      write!(f, " {}\n", title)?;
      for node in children {
        let mut level_next = level.clone();
        level_next.push(deep);
        deep -= 1;
        write_ascii_node(f, node, &level_next)?;
      }
    }
    AsciiNode::Leaf(lines) => {
      for (i, line) in lines.iter().enumerate() {
        match i {
          0 => writeln!(f, " {}", line)?,
          _ => writeln!(f, "{} {}", second_line, line)?,
        }
      }
    }
  }
  Ok(())
}

pub struct AsciiLeafBuilder(Vec<AsciiLine>);

impl AsciiLeafBuilder {
  ///
  pub fn new() -> Self {
    Self(vec![])
  }

  ///
  pub fn line(mut self, line: AsciiLine) -> Self {
    self.0.push(line);
    self
  }

  ///
  pub fn build(self) -> AsciiNode {
    AsciiNode::Leaf(self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{ascii256, ASCII_CYAN};

  #[test]
  fn a() {
    let t3 = AsciiText::new("line3");
    let t4 = AsciiText::new("line4");

    let t5 = AsciiText::new("only one line");

    let l3 = AsciiLine::new(vec![t3]);
    let l4 = AsciiLine::new(vec![t4]);

    let l5 = AsciiLine::new(vec![t5]);

    let leaf1 = AsciiNode::leaf_builder()
      .line(AsciiLine::builder().with_color("line1", ASCII_CYAN).with_color("continuation", &ascii256!(187)).build())
      .line(AsciiLine::builder().text("line2").space().with_color("continuation", &ascii256!(30)).build())
      .line(l3)
      .line(l4)
      .build();

    let leaf2 = AsciiNode::Leaf(vec![l5]);

    let node1 = AsciiNode::Node(AsciiLine::builder().text("node 1").build(), vec![leaf1, leaf2]);

    // let l1 = AsciiNode::Leaf(vec![String::from("line1"), String::from("line2"), String::from("line3"), String::from("line4")]);
    // let l2 = AsciiNode::Leaf(vec![String::from("only one line")]);
    // let n1 = AsciiNode::Node(String::from("node 1"), vec![l1.clone(), l2.clone()]);
    // let n2 = AsciiNode::Node(String::from("node 2"), vec![l2.clone(), l1.clone(), l2.clone()]);
    // let n3 = AsciiNode::Node(String::from("node 3"), vec![n1.clone(), l1.clone(), l2.clone()]);
    // let n4 = AsciiNode::Node(String::from("node 4"), vec![n1, n2, n3]);
    let mut output = String::new();
    //let _ = write_ascii_tree(&mut output, &n4);
    let _ = write_ascii_tree(&mut output, &node1);
    println!("{}", output);
  }
}
