/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use crate::*;
use bitflags::bitflags;
use core::fmt::Debug;
use crossterm::style::Color;
use r3bl_rs_utils::{unwrap_option_or_compute_if_none, Builder};
use serde::{Deserialize, Serialize};

use std::fmt::Formatter;
use std::ops::{Add, AddAssign};

/// Use the `StyleBuilder` to create a `Style`. `Style` objects are meant to be immutable.
/// If you need to modify a `Style`, you should use the `StyleBuilder` to create a new
/// one.
#[derive(Default, Builder, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
  pub id: String,
  pub bold: bool,
  pub dim: bool,
  pub underline: bool,
  pub reverse: bool,
  pub hidden: bool,
  pub fraktur: bool,
  pub computed: bool,
  pub color_fg: Option<TWColor>,
  pub color_bg: Option<TWColor>,
  pub margin: Option<UnitType>,
  pub cached_bitflags: Option<StyleFlag>,
}

enum DebugColor {
  None,
  DebugRgb(Color),
}

impl DebugColor {
  fn from(color: Option<TWColor>) -> DebugColor {
    match color {
      Some(color) => DebugColor::DebugRgb(*color),
      None => DebugColor::None,
    }
  }
}

impl Debug for DebugColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if let DebugColor::None = self {
      return Ok(());
    }
    if let DebugColor::DebugRgb(Color::Rgb { r, g, b }) = self {
      f.write_fmt(format_args!("{},{},{}", r, g, b))
    } else {
      f.write_fmt(format_args!("{:?}", self))
    }
  }
}

impl Debug for Style {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    let mut msg_vec: Vec<String> = vec![];
    if self.computed {
      msg_vec.push("computed".to_string())
    } else {
      msg_vec.push(self.id.to_string());
    }
    if self.bold {
      msg_vec.push("bold".to_string())
    }
    if self.dim {
      msg_vec.push("dim".to_string())
    }
    if self.underline {
      msg_vec.push("underline".to_string())
    }
    if self.reverse {
      msg_vec.push("reverse".to_string())
    }
    if self.hidden {
      msg_vec.push("hidden".to_string())
    }
    if self.fraktur {
      msg_vec.push("fraktur".to_string())
    }

    write!(
      f,
      "Style {{ {} | fg: {:?} | bg: {:?} | margin: {:?} }}",
      msg_vec.join("+"),
      DebugColor::from(self.color_fg.clone()),
      DebugColor::from(self.color_bg.clone()),
      if self.margin.is_some() { self.margin.unwrap() } else { 0 }
    )
  }
}

bitflags! {
  /// https://docs.rs/bitflags/0.8.2/bitflags/macro.bitflags.html
  #[derive(Serialize, Deserialize)]
  pub struct StyleFlag: u8 {
    const COLOR_FG_SET        = 0b0000_0001;
    const COLOR_BG_SET        = 0b0000_0010;
    const BOLD_SET            = 0b0000_0100;
    const DIM_SET             = 0b0000_1000;
    const UNDERLINE_SET       = 0b0001_0000;
    const MARGIN_SET          = 0b0010_0000;
    const COMPUTED_SET        = 0b0100_0000;
    const REVERSE_SET         = 0b1000_0000;
    const HIDDEN_SET          = 0b1000_0001;
    const STRIKETHROUGH_SET   = 0b1000_0010;
  }
}

impl Style {
  /// The `StyleFlag` is lazily computed and cached after the first time it is evaluated.
  /// A `Style` should be built using via `StyleBuilder and the expectation is that once
  /// built, the style won't be modified.
  pub fn get_bitflags(&mut self) -> StyleFlag {
    unwrap_option_or_compute_if_none! {
      self.cached_bitflags,
      || self.gen_bitflags()
    }
  }

  pub fn reset_bitflags(&mut self) {
    self.cached_bitflags = None;
  }

  fn gen_bitflags(&self) -> StyleFlag {
    let mut it = StyleFlag::empty();

    if self.color_fg.is_some() {
      it.insert(StyleFlag::COLOR_FG_SET);
    }
    if self.color_bg.is_some() {
      it.insert(StyleFlag::COLOR_BG_SET);
    }
    if self.margin.is_some() {
      it.insert(StyleFlag::MARGIN_SET);
    }
    if self.bold {
      it.insert(StyleFlag::BOLD_SET);
    }
    if self.dim {
      it.insert(StyleFlag::DIM_SET);
    }
    if self.underline {
      it.insert(StyleFlag::UNDERLINE_SET);
    }
    if self.computed {
      it.insert(StyleFlag::COMPUTED_SET);
    }

    it
  }
}

/// Implement specificity behavior for [Style] by implementing [Add] trait. Here's the
/// rule: `Style + Style (overrides) = Style`.
/// - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
impl Add<Self> for Style {
  type Output = Self;

  fn add(mut self, other: Self) -> Self {
    // Computed style has no id.
    self.computed = true;
    self.id = "".to_string();

    // other (if set) overrides self.
    if let Some(color_fg) = other.color_fg {
      self.color_fg = Some(color_fg);
    }
    if let Some(color_bg) = other.color_bg {
      self.color_bg = Some(color_bg);
    }
    if let Some(margin) = other.margin {
      self.margin = Some(margin);
    }
    if other.bold {
      self.bold = true;
    }
    if other.dim {
      self.dim = true;
    }
    if other.underline {
      self.underline = true;
    }

    // Recalculate the bitflags.
    self.reset_bitflags();
    self.get_bitflags();

    self
  }
}

impl AddAssign<&Style> for Style {
  fn add_assign(&mut self, other: &Style) {
    *self = self.clone() + other.clone();
  }
}
