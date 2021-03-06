// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{c_int, c_uint, c_char, c_double};
use gtk::ffi::{Gboolean};
use pango;

#[repr(C)]
#[derive(Copy)]
pub struct C_PangoContext;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAttrList;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAttrIterator;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoItem;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontMap;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontDescription;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoLanguage;
#[repr(C)]
#[derive(Copy)]
pub struct PangoMatrix {
    pub xx: c_double,
    pub xy: c_double,
    pub yx: c_double,
    pub yy: c_double,
    pub x0: c_double,
    pub y0: c_double
}
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFont;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontset;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontMetrics;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontFamily;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAnalysis;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoLogAttr;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoGlyphString;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoScript;
#[repr(C)]
#[derive(Copy)]
pub struct PangoRectangle {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}

extern "C" {
    //=========================================================================
    // PangoItem                                                         NOT OK
    //=========================================================================
    pub fn pango_item_free                (item: *mut C_PangoItem);
    pub fn pango_item_copy                (item: *mut C_PangoItem) -> *mut C_PangoItem;
    pub fn pango_item_new                 () -> *mut C_PangoItem;
    pub fn pango_item_split               (item: *mut C_PangoItem, split_index: c_int, split_offset: c_int) -> *mut C_PangoItem;

    //=========================================================================
    // PangoContext                                                      NOT OK
    //=========================================================================
    pub fn pango_context_new              () -> *mut C_PangoContext;
    pub fn pango_context_changed          (context: *mut C_PangoContext);
    pub fn pango_context_get_serial       (context: *mut C_PangoContext) -> c_uint;
    pub fn pango_context_set_font_map     (context: *mut C_PangoContext, font_map: *mut C_PangoFontMap);
    pub fn pango_context_get_font_map     (context: *mut C_PangoContext) -> *mut C_PangoFontMap;
    pub fn pango_context_get_font_description(context: *mut C_PangoContext) -> *mut C_PangoFontDescription;
    pub fn pango_context_set_font_description(context: *mut C_PangoContext, desc: *const C_PangoFontDescription);
    pub fn pango_context_get_language     (context: *mut C_PangoContext) -> *mut C_PangoLanguage;
    pub fn pango_context_set_language     (context: *mut C_PangoContext, language: *mut C_PangoLanguage);
    pub fn pango_context_get_base_dir     (context: *mut C_PangoContext) -> pango::Direction;
    pub fn pango_context_set_base_dir     (context: *mut C_PangoContext, direction: pango::Direction);
    pub fn pango_context_get_base_gravity (context: *mut C_PangoContext) -> pango::Gravity;
    pub fn pango_context_set_base_gravity (context: *mut C_PangoContext, gravity: pango::Gravity);
    pub fn pango_context_get_gravity      (context: *mut C_PangoContext) -> pango::Gravity;
    pub fn pango_context_get_gravity_hint (context: *mut C_PangoContext) -> pango::GravityHint;
    pub fn pango_context_set_gravity_hint (context: *mut C_PangoContext, hint: pango::GravityHint);
    pub fn pango_context_get_matrix       (context: *mut C_PangoContext) -> *const PangoMatrix;
    pub fn pango_context_set_matrix       (context: *mut C_PangoContext, matrix: *const PangoMatrix);
    pub fn pango_context_load_font        (context: *mut C_PangoContext, desc: *const C_PangoFontDescription) -> *mut C_PangoFont;
    pub fn pango_context_load_fontset     (context: *mut C_PangoContext, desc: *const C_PangoFontDescription, language: *mut C_PangoLanguage) -> *mut C_PangoFontset;
    pub fn pango_context_get_metrics      (context: *mut C_PangoContext, desc: *const C_PangoFontDescription, language: *mut C_PangoLanguage) -> *mut C_PangoFontMetrics;
    pub fn pango_context_list_families    (context: *mut C_PangoContext, families: *mut *mut *mut C_PangoFontFamily, n_families: *mut c_int);

    //=========================================================================
    // PangoFontDescription                                              NOT OK
    //=========================================================================
    pub fn pango_font_description_new     () -> *mut C_PangoFontDescription;
    pub fn pango_font_description_copy    (desc: *const C_PangoFontDescription) -> *mut C_PangoFontDescription;
    //pub fn pango_font_description_copy_static(desc: *const C_PangoFontDescription) -> *mut C_PangoFontDescription;
    pub fn pango_font_description_hash    (desc: *const C_PangoFontDescription) -> c_uint;
    pub fn pango_font_description_equal   (desc1: *const C_PangoFontDescription, desc2: *const C_PangoFontDescription) -> Gboolean;
    pub fn pango_font_description_free    (desc: *mut C_PangoFontDescription);
    pub fn pango_font_descriptions_free   (desc: *mut *mut C_PangoFontDescription, n_descs: c_int);
    pub fn pango_font_description_set_family(desc: *mut C_PangoFontDescription, family: *const c_char);
    //pub fn pango_font_description_set_family_static(desc: *mut C_PangoFontDescription, family: *const c_char);
    pub fn pango_font_description_get_family(desc: *const C_PangoFontDescription) -> *const c_char;
    pub fn pango_font_description_set_style(desc: *mut C_PangoFontDescription, style: pango::Style);
    pub fn pango_font_description_get_style(desc: *const C_PangoFontDescription) -> pango::Style;
    pub fn pango_font_description_set_variant(desc: *mut C_PangoFontDescription, variant: pango::Variant);
    pub fn pango_font_description_get_variant(desc: *const C_PangoFontDescription) -> pango::Variant;
    pub fn pango_font_description_set_weight(desc: *mut C_PangoFontDescription, weight: pango::Weight);
    pub fn pango_font_description_get_weight(desc: *const C_PangoFontDescription) -> pango::Weight;
    pub fn pango_font_description_set_stretch(desc: *mut C_PangoFontDescription, stretch: pango::Stretch);
    pub fn pango_font_description_get_stretch(desc: *const C_PangoFontDescription) -> pango::Stretch;
    pub fn pango_font_description_set_size(desc: *mut C_PangoFontDescription, size: c_int);
    pub fn pango_font_description_get_size(desc: *const C_PangoFontDescription) -> c_int;
    pub fn pango_font_description_set_absolute_size(desc: *mut C_PangoFontDescription, size: c_double);
    pub fn pango_font_description_get_size_is_absolute(desc: *const C_PangoFontDescription) -> Gboolean;
    pub fn pango_font_description_set_gravity(desc: *mut C_PangoFontDescription, gravity: pango::Gravity);
    pub fn pango_font_description_get_gravity(desc: *const C_PangoFontDescription) -> pango::Gravity;
    pub fn pango_font_description_get_set_fields(desc: *const C_PangoFontDescription) -> pango::FontMask;
    pub fn pango_font_description_unset_fields(desc: *mut C_PangoFontDescription, to_unset: pango::FontMask);
    pub fn pango_font_description_merge   (desc: *mut C_PangoFontDescription, desc_to_merge: *const C_PangoFontDescription,
        replace_existing: Gboolean);
    //pub fn pango_font_description_merge_static(desc: *mut C_PangoFontDescription, desc_to_merge: *const C_PangoFontDescription,
    //    replace_existing: Gboolean);
    pub fn pango_font_description_better_match(desc: *const C_PangoFontDescription, old_match: *const C_PangoFontDescription,
        new_match: *const C_PangoFontDescription) -> Gboolean;
    pub fn pango_font_description_from_string(str_: *const c_char) -> *mut C_PangoFontDescription;
    pub fn pango_font_description_to_string(desc: *const C_PangoFontDescription) -> *mut c_char;
    pub fn pango_font_description_to_filename(desc: *const C_PangoFontDescription) -> *mut c_char;

    //=========================================================================
    // PangoMatrix                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_matrix    (matrix: *const PangoMatrix) -> pango::Gravity;
    //pub fn pango_matrix_copy               (matrix: *const PangoMatrix) -> *mut PangoMatrix;
    //pub fn pango_matrix_free               (matrix: *mut PangoMatrix);
    pub fn pango_matrix_translate          (matrix: *mut PangoMatrix, t_x: c_double, t_y: c_double);
    pub fn pango_matrix_scale              (matrix: *mut PangoMatrix, scale_x: c_double, scale_y: c_double);
    pub fn pango_matrix_rotate             (matrix: *mut PangoMatrix, degrees: c_double);
    pub fn pango_matrix_concat             (matrix: *mut PangoMatrix, new_matrix: *const PangoMatrix);
    pub fn pango_matrix_transform_point    (matrix: *const PangoMatrix, x: *mut c_double, y: *mut c_double);
    pub fn pango_matrix_transform_distance (matrix: *const PangoMatrix, dx: *mut c_double, dy: *mut c_double);
    pub fn pango_matrix_transform_rectangle(matrix: *const PangoMatrix, rect: *mut PangoRectangle);
    pub fn pango_matrix_transform_pixel_rectangle(matrix: *const PangoMatrix, rect: *mut PangoRectangle);
    pub fn pango_matrix_get_font_scale_factor(matrix: *mut PangoMatrix) -> c_double;

    //=========================================================================
    // PangoGlyphString                                                  NOT OK
    //=========================================================================
    pub fn pango_glyph_string_new          () -> *mut C_PangoGlyphString;
    pub fn pango_glyph_string_copy         (string: *mut C_PangoGlyphString) -> *mut C_PangoGlyphString;
    pub fn pango_glyph_string_set_size     (string: *mut C_PangoGlyphString, new_len: c_int);
    pub fn pango_glyph_string_free         (string: *mut C_PangoGlyphString);
    pub fn pango_glyph_string_extents      (string: *mut C_PangoGlyphString, font: *mut C_PangoFont, ink_rect: *mut PangoRectangle,
        logical_rect: *mut PangoRectangle);
    pub fn pango_glyph_string_extents_range(string: *mut C_PangoGlyphString, start: c_int, end: c_int, font: *mut C_PangoFont,
        ink_rect: *mut PangoRectangle, logical_rect: *mut PangoRectangle);
    pub fn pango_glyph_string_get_width    (string: *mut C_PangoGlyphString) -> c_int;
    pub fn pango_glyph_string_index_to_x   (string: *mut C_PangoGlyphString, text: *mut c_char, length: c_int, analysis: *mut C_PangoAnalysis,
        index_: c_int, trailing: Gboolean, x_pos: *mut c_int);
    pub fn pango_glyph_string_x_to_index   (string: *mut C_PangoGlyphString, text: *mut c_char, length: c_int, analysis: *mut C_PangoAnalysis,
        x_pos: c_int, index_: *mut c_int, trailing: *mut c_int);
    pub fn pango_glyph_string_get_logical_widths(glyphs: *mut C_PangoGlyphString, text: *mut c_char, length: c_int, embedding_level: c_int,
        logical_widths: *mut c_int);

    //=========================================================================
    // PangoScript                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_script    (script: pango::Script, base_gravity: pango::Gravity, hint: pango::GravityHint) -> pango::Gravity;
    pub fn pango_gravity_get_for_script_and_width(script: pango::Script, wide: Gboolean, base_gravity: pango::Gravity,
        hint: pango::GravityHint) -> pango::Gravity;

    //=========================================================================
    // PangoGravity                                                      NOT OK
    //=========================================================================
    pub fn pango_gravity_to_rotation        (gravity: pango::Gravity) -> c_double;

    //=========================================================================
    // PangoDirection                                                    NOT OK
    //=========================================================================
    pub fn pango_unichar_direction          (ch: u32) -> pango::Direction;
    pub fn pango_find_base_dir              (text: *const c_char, length: c_int) -> pango::Direction;

    //=========================================================================
    // PangoBidiType                                                     NOT OK
    //=========================================================================
    pub fn pango_bidi_type_for_unichar      (ch: u32) -> pango::BidiType;

    //pub fn pango_itemize                  (context: *mut C_PangoContext, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut C_PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_itemize_with_base_dir    (context: *mut C_PangoContext, direction: pango::Direction, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut C_PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_reorder_items            (logical_items: *mut GList) -> *mut GList;
    pub fn pango_break                    (text: *const c_char, length: c_int, analysis: *mut C_PangoAnalysis, attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_get_log_attrs            (text: *const c_char, length: c_int, level: c_int, language: *mut C_PangoLanguage, log_attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_find_paragraph_boundary  (text: *const c_char, length: c_int, paragraph_delimiter_index: *mut c_int,
        next_paragraph_start: *mut c_int);
    pub fn pango_default_break            (text: *const c_char, length: c_int, analysis: *mut C_PangoAnalysis, attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_shape                    (text: *const c_char, length: c_int, analysis: *const C_PangoAnalysis, glyphs: *mut C_PangoGlyphString);
    pub fn pango_shape_full               (item_text: *const c_char, item_length: c_int, paragraph_text: *const c_char, paragraph_length: c_int,
        analysis: *const C_PangoAnalysis, glyphs: *mut C_PangoGlyphString);
}