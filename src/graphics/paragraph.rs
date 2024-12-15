use super::color::Color;
use crate::math::Vec2;
use skia_safe::{
    font_style::{Slant, Weight, Width},
    textlayout::{
        FontCollection, Paragraph as SkParagraph, ParagraphBuilder,
        ParagraphStyle as SkParagraphStyle, TextAlign, TextStyle,
    },
    FontMgr, FontStyle,
};
use std::cell::RefCell;

pub struct ParagraphStyle {
    pub size: f32,
    pub color: Color,
}

pub struct Paragraph {
    pub(super) sk_paragraph: RefCell<SkParagraph>,
}

impl Paragraph {
    pub fn new(text: impl AsRef<str>, style: ParagraphStyle) -> Self {
        let mut text_style = TextStyle::new();
        text_style.set_font_size(style.size);
        text_style.set_letter_spacing(1.2);
        text_style.set_color(skia_safe::Color::from_argb(
            style.color.a,
            style.color.r,
            style.color.g,
            style.color.b,
        ));
        text_style.set_font_families(&["Helvetica Neue"]);
        text_style.set_font_style(FontStyle::new(
            Weight::NORMAL,
            Width::NORMAL,
            Slant::Upright,
        ));

        let mut par_style = SkParagraphStyle::new();
        par_style.set_text_align(TextAlign::Justify);

        let font_collection = FONT_COLLECTION.with(|collection| collection.clone());
        let paragraph = ParagraphBuilder::new(&par_style, font_collection)
            .push_style(&text_style)
            .add_text(text)
            .build();

        Self {
            sk_paragraph: RefCell::new(paragraph),
        }
    }

    pub fn size(&self, width: f32) -> Vec2 {
        let mut sk_paragraph = self.sk_paragraph.borrow_mut();
        if sk_paragraph.max_width() != width {
            sk_paragraph.layout(width);
        }

        Vec2::new(
            width.min(sk_paragraph.max_intrinsic_width().ceil()),
            sk_paragraph.height(),
        )
    }
}

thread_local! {
    static FONT_COLLECTION: FontCollection = {
        let mut collection = FontCollection::new();
        collection.set_asset_font_manager(Some(FontMgr::new()));
        collection
    };
}
