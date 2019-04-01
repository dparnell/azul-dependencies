// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This file is autogenerated. Do not edit it!

use std::fmt;

/// List of all SVG elements.
#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Hash)]
#[allow(missing_docs)]
pub enum ElementId {
    A,
    AltGlyph,
    AltGlyphDef,
    AltGlyphItem,
    Animate,
    AnimateColor,
    AnimateMotion,
    AnimateTransform,
    Circle,
    ClipPath,
    ColorProfile,
    Cursor,
    Defs,
    Desc,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    Font,
    FontFace,
    FontFaceFormat,
    FontFaceName,
    FontFaceSrc,
    FontFaceUri,
    ForeignObject,
    G,
    Glyph,
    GlyphRef,
    Hkern,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Metadata,
    MissingGlyph,
    Mpath,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Script,
    Set,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tref,
    Tspan,
    Use,
    View,
    Vkern
}

static ELEMENTS: ::phf::Map<&'static str, ElementId> = ::phf::Map {
    key: 1897749892740154578,
    disps: ::phf::Slice::Static(&[
        (0, 19),
        (1, 2),
        (5, 9),
        (35, 31),
        (0, 43),
        (1, 12),
        (0, 1),
        (1, 61),
        (12, 20),
        (0, 26),
        (16, 35),
        (0, 4),
        (2, 52),
        (0, 38),
        (0, 0),
        (39, 41),
    ]),
    entries: ::phf::Slice::Static(&[
        ("svg", ElementId::Svg),
        ("radialGradient", ElementId::RadialGradient),
        ("font-face-src", ElementId::FontFaceSrc),
        ("font-face", ElementId::FontFace),
        ("feFlood", ElementId::FeFlood),
        ("desc", ElementId::Desc),
        ("metadata", ElementId::Metadata),
        ("switch", ElementId::Switch),
        ("font-face-name", ElementId::FontFaceName),
        ("feSpotLight", ElementId::FeSpotLight),
        ("feComponentTransfer", ElementId::FeComponentTransfer),
        ("font", ElementId::Font),
        ("ellipse", ElementId::Ellipse),
        ("mpath", ElementId::Mpath),
        ("a", ElementId::A),
        ("feMerge", ElementId::FeMerge),
        ("polyline", ElementId::Polyline),
        ("line", ElementId::Line),
        ("feFuncA", ElementId::FeFuncA),
        ("feGaussianBlur", ElementId::FeGaussianBlur),
        ("feDisplacementMap", ElementId::FeDisplacementMap),
        ("feImage", ElementId::FeImage),
        ("linearGradient", ElementId::LinearGradient),
        ("symbol", ElementId::Symbol),
        ("style", ElementId::Style),
        ("altGlyphItem", ElementId::AltGlyphItem),
        ("feTile", ElementId::FeTile),
        ("defs", ElementId::Defs),
        ("view", ElementId::View),
        ("polygon", ElementId::Polygon),
        ("circle", ElementId::Circle),
        ("animateTransform", ElementId::AnimateTransform),
        ("glyph", ElementId::Glyph),
        ("color-profile", ElementId::ColorProfile),
        ("text", ElementId::Text),
        ("feMorphology", ElementId::FeMorphology),
        ("fePointLight", ElementId::FePointLight),
        ("feConvolveMatrix", ElementId::FeConvolveMatrix),
        ("feFuncR", ElementId::FeFuncR),
        ("feDiffuseLighting", ElementId::FeDiffuseLighting),
        ("path", ElementId::Path),
        ("font-face-format", ElementId::FontFaceFormat),
        ("g", ElementId::G),
        ("cursor", ElementId::Cursor),
        ("title", ElementId::Title),
        ("animateMotion", ElementId::AnimateMotion),
        ("feBlend", ElementId::FeBlend),
        ("feSpecularLighting", ElementId::FeSpecularLighting),
        ("altGlyph", ElementId::AltGlyph),
        ("marker", ElementId::Marker),
        ("tref", ElementId::Tref),
        ("feFuncG", ElementId::FeFuncG),
        ("feOffset", ElementId::FeOffset),
        ("set", ElementId::Set),
        ("feDistantLight", ElementId::FeDistantLight),
        ("clipPath", ElementId::ClipPath),
        ("missing-glyph", ElementId::MissingGlyph),
        ("feMergeNode", ElementId::FeMergeNode),
        ("pattern", ElementId::Pattern),
        ("foreignObject", ElementId::ForeignObject),
        ("animate", ElementId::Animate),
        ("feTurbulence", ElementId::FeTurbulence),
        ("script", ElementId::Script),
        ("textPath", ElementId::TextPath),
        ("use", ElementId::Use),
        ("feColorMatrix", ElementId::FeColorMatrix),
        ("rect", ElementId::Rect),
        ("image", ElementId::Image),
        ("font-face-uri", ElementId::FontFaceUri),
        ("hkern", ElementId::Hkern),
        ("filter", ElementId::Filter),
        ("altGlyphDef", ElementId::AltGlyphDef),
        ("animateColor", ElementId::AnimateColor),
        ("tspan", ElementId::Tspan),
        ("feFuncB", ElementId::FeFuncB),
        ("stop", ElementId::Stop),
        ("glyphRef", ElementId::GlyphRef),
        ("mask", ElementId::Mask),
        ("feComposite", ElementId::FeComposite),
        ("vkern", ElementId::Vkern),
    ]),
};

impl ElementId {
    /// Converts name into id.
    pub fn from_str(text: &str) -> Option<ElementId> {
        ELEMENTS.get(text).cloned()
    }

    /// Converts id into name.
    pub fn as_str(&self) -> &str {
        match *self {
            ElementId::A => "a",
            ElementId::AltGlyph => "altGlyph",
            ElementId::AltGlyphDef => "altGlyphDef",
            ElementId::AltGlyphItem => "altGlyphItem",
            ElementId::Animate => "animate",
            ElementId::AnimateColor => "animateColor",
            ElementId::AnimateMotion => "animateMotion",
            ElementId::AnimateTransform => "animateTransform",
            ElementId::Circle => "circle",
            ElementId::ClipPath => "clipPath",
            ElementId::ColorProfile => "color-profile",
            ElementId::Cursor => "cursor",
            ElementId::Defs => "defs",
            ElementId::Desc => "desc",
            ElementId::Ellipse => "ellipse",
            ElementId::FeBlend => "feBlend",
            ElementId::FeColorMatrix => "feColorMatrix",
            ElementId::FeComponentTransfer => "feComponentTransfer",
            ElementId::FeComposite => "feComposite",
            ElementId::FeConvolveMatrix => "feConvolveMatrix",
            ElementId::FeDiffuseLighting => "feDiffuseLighting",
            ElementId::FeDisplacementMap => "feDisplacementMap",
            ElementId::FeDistantLight => "feDistantLight",
            ElementId::FeFlood => "feFlood",
            ElementId::FeFuncA => "feFuncA",
            ElementId::FeFuncB => "feFuncB",
            ElementId::FeFuncG => "feFuncG",
            ElementId::FeFuncR => "feFuncR",
            ElementId::FeGaussianBlur => "feGaussianBlur",
            ElementId::FeImage => "feImage",
            ElementId::FeMerge => "feMerge",
            ElementId::FeMergeNode => "feMergeNode",
            ElementId::FeMorphology => "feMorphology",
            ElementId::FeOffset => "feOffset",
            ElementId::FePointLight => "fePointLight",
            ElementId::FeSpecularLighting => "feSpecularLighting",
            ElementId::FeSpotLight => "feSpotLight",
            ElementId::FeTile => "feTile",
            ElementId::FeTurbulence => "feTurbulence",
            ElementId::Filter => "filter",
            ElementId::Font => "font",
            ElementId::FontFace => "font-face",
            ElementId::FontFaceFormat => "font-face-format",
            ElementId::FontFaceName => "font-face-name",
            ElementId::FontFaceSrc => "font-face-src",
            ElementId::FontFaceUri => "font-face-uri",
            ElementId::ForeignObject => "foreignObject",
            ElementId::G => "g",
            ElementId::Glyph => "glyph",
            ElementId::GlyphRef => "glyphRef",
            ElementId::Hkern => "hkern",
            ElementId::Image => "image",
            ElementId::Line => "line",
            ElementId::LinearGradient => "linearGradient",
            ElementId::Marker => "marker",
            ElementId::Mask => "mask",
            ElementId::Metadata => "metadata",
            ElementId::MissingGlyph => "missing-glyph",
            ElementId::Mpath => "mpath",
            ElementId::Path => "path",
            ElementId::Pattern => "pattern",
            ElementId::Polygon => "polygon",
            ElementId::Polyline => "polyline",
            ElementId::RadialGradient => "radialGradient",
            ElementId::Rect => "rect",
            ElementId::Script => "script",
            ElementId::Set => "set",
            ElementId::Stop => "stop",
            ElementId::Style => "style",
            ElementId::Svg => "svg",
            ElementId::Switch => "switch",
            ElementId::Symbol => "symbol",
            ElementId::Text => "text",
            ElementId::TextPath => "textPath",
            ElementId::Title => "title",
            ElementId::Tref => "tref",
            ElementId::Tspan => "tspan",
            ElementId::Use => "use",
            ElementId::View => "view",
            ElementId::Vkern => "vkern"
        }
    }
}

impl fmt::Debug for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
