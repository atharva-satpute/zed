// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn summercamp() -> UserThemeFamily {
    UserThemeFamily {
        name: "Summercamp".into(),
        author: "Zed Industries".into(),
        themes: vec![UserTheme {
            name: "Summercamp".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x312d21ff).into()),
                    border_variant: Some(rgba(0x312d21ff).into()),
                    border_focused: Some(rgba(0x193761ff).into()),
                    border_selected: Some(rgba(0x193761ff).into()),
                    border_transparent: Some(rgba(0x00000000).into()),
                    border_disabled: Some(rgba(0x2e2a1fff).into()),
                    elevated_surface_background: Some(rgba(0x231f16ff).into()),
                    surface_background: Some(rgba(0x231f16ff).into()),
                    background: Some(rgba(0x2a261cff).into()),
                    panel_background: Some(rgba(0x231f16ff).into()),
                    element_background: Some(rgba(0x231f16ff).into()),
                    element_hover: Some(rgba(0x29251bff).into()),
                    element_active: Some(rgba(0x302c20ff).into()),
                    element_selected: Some(rgba(0x302c20ff).into()),
                    element_disabled: Some(rgba(0x231f16ff).into()),
                    drop_target_background: Some(rgba(0x736e5580).into()),
                    ghost_element_background: Some(rgba(0x00000000).into()),
                    ghost_element_hover: Some(rgba(0x29251bff).into()),
                    ghost_element_active: Some(rgba(0x302c20ff).into()),
                    ghost_element_selected: Some(rgba(0x302c20ff).into()),
                    ghost_element_disabled: Some(rgba(0x231f16ff).into()),
                    text: Some(rgba(0xf8f5deff).into()),
                    text_muted: Some(rgba(0x736e55ff).into()),
                    text_placeholder: Some(rgba(0x4c4735ff).into()),
                    text_disabled: Some(rgba(0x4c4735ff).into()),
                    text_accent: Some(rgba(0x499befff).into()),
                    icon: Some(rgba(0xf8f5deff).into()),
                    icon_muted: Some(rgba(0x736e55ff).into()),
                    icon_disabled: Some(rgba(0x4c4735ff).into()),
                    icon_placeholder: Some(rgba(0x736e55ff).into()),
                    icon_accent: Some(rgba(0x499befff).into()),
                    status_bar_background: Some(rgba(0x2a261cff).into()),
                    title_bar_background: Some(rgba(0x2a261cff).into()),
                    toolbar_background: Some(rgba(0x1c1810ff).into()),
                    tab_bar_background: Some(rgba(0x231f16ff).into()),
                    tab_inactive_background: Some(rgba(0x231f16ff).into()),
                    tab_active_background: Some(rgba(0x1c1810ff).into()),
                    scrollbar_thumb_background: Some(rgba(0xf8f5de4c).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0x29251bff).into()),
                    scrollbar_thumb_border: Some(rgba(0x29251bff).into()),
                    scrollbar_track_background: Some(rgba(0x00000000).into()),
                    scrollbar_track_border: Some(rgba(0x221e15ff).into()),
                    editor_foreground: Some(rgba(0xf8f5deff).into()),
                    editor_background: Some(rgba(0x1c1810ff).into()),
                    editor_gutter_background: Some(rgba(0x1c1810ff).into()),
                    editor_subheader_background: Some(rgba(0x231f16ff).into()),
                    editor_active_line_background: Some(rgba(0x231f16bf).into()),
                    editor_highlighted_line_background: Some(rgba(0x231f16ff).into()),
                    editor_line_number: Some(rgba(0xf8f5de59).into()),
                    editor_active_line_number: Some(rgba(0xf8f5deff).into()),
                    editor_invisible: Some(rgba(0x736e55ff).into()),
                    editor_wrap_guide: Some(rgba(0xf8f5de0d).into()),
                    editor_active_wrap_guide: Some(rgba(0xf8f5de1a).into()),
                    editor_document_highlight_read_background: Some(rgba(0x499bef1a).into()),
                    editor_document_highlight_write_background: Some(rgba(0x49443366).into()),
                    terminal_background: Some(rgba(0x1c1810ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x3b3627ff).into()),
                    terminal_ansi_bright_red: Some(rgba(0x7f2724ff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x28842cff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0x8c9a10ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x234b7fff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0x88487eff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x298462ff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xf8f5deff).into()),
                    terminal_ansi_black: Some(rgba(0x1c1810ff).into()),
                    terminal_ansi_red: Some(rgba(0xe35142ff).into()),
                    terminal_ansi_green: Some(rgba(0x5dea5aff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf1fe29ff).into()),
                    terminal_ansi_blue: Some(rgba(0x499befff).into()),
                    terminal_ansi_magenta: Some(rgba(0xf59be6ff).into()),
                    terminal_ansi_cyan: Some(rgba(0x5beabcff).into()),
                    terminal_ansi_white: Some(rgba(0xf8f5deff).into()),
                    link_text_hover: Some(rgba(0x499befff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    conflict: Some(rgba(0xf1fe29ff).into()),
                    conflict_background: Some(rgba(0x556305ff).into()),
                    conflict_border: Some(rgba(0x727f0aff).into()),
                    created: Some(rgba(0x5dea5aff).into()),
                    created_background: Some(rgba(0x0a4d13ff).into()),
                    created_border: Some(rgba(0x1a6a20ff).into()),
                    deleted: Some(rgba(0xe35142ff).into()),
                    deleted_background: Some(rgba(0x491013ff).into()),
                    deleted_border: Some(rgba(0x651c1cff).into()),
                    error: Some(rgba(0xe35142ff).into()),
                    error_background: Some(rgba(0x491013ff).into()),
                    error_border: Some(rgba(0x651c1cff).into()),
                    hidden: Some(rgba(0x4c4735ff).into()),
                    hidden_background: Some(rgba(0x2a261cff).into()),
                    hidden_border: Some(rgba(0x2e2a1fff).into()),
                    hint: Some(rgba(0x246e61ff).into()),
                    hint_background: Some(rgba(0x0e2242ff).into()),
                    hint_border: Some(rgba(0x193761ff).into()),
                    ignored: Some(rgba(0x736e55ff).into()),
                    ignored_background: Some(rgba(0x2a261cff).into()),
                    ignored_border: Some(rgba(0x312d21ff).into()),
                    info: Some(rgba(0x499befff).into()),
                    info_background: Some(rgba(0x0e2242ff).into()),
                    info_border: Some(rgba(0x193761ff).into()),
                    modified: Some(rgba(0xf1fe29ff).into()),
                    modified_background: Some(rgba(0x556305ff).into()),
                    modified_border: Some(rgba(0x727f0aff).into()),
                    predictive: Some(rgba(0x79434bff).into()),
                    predictive_background: Some(rgba(0x0a4d13ff).into()),
                    predictive_border: Some(rgba(0x1a6a20ff).into()),
                    renamed: Some(rgba(0x499befff).into()),
                    renamed_background: Some(rgba(0x0e2242ff).into()),
                    renamed_border: Some(rgba(0x193761ff).into()),
                    success: Some(rgba(0x5dea5aff).into()),
                    success_background: Some(rgba(0x0a4d13ff).into()),
                    success_border: Some(rgba(0x1a6a20ff).into()),
                    unreachable: Some(rgba(0x736e55ff).into()),
                    unreachable_background: Some(rgba(0x2a261cff).into()),
                    unreachable_border: Some(rgba(0x312d21ff).into()),
                    warning: Some(rgba(0xf1fe29ff).into()),
                    warning_background: Some(rgba(0x556305ff).into()),
                    warning_border: Some(rgba(0x727f0aff).into()),
                    ..Default::default()
                },
                player: Some(PlayerColors(vec![
                    PlayerColor {
                        cursor: rgba(0x499befff).into(),
                        background: rgba(0x499befff).into(),
                        selection: rgba(0x499bef3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf59be6ff).into(),
                        background: rgba(0xf59be6ff).into(),
                        selection: rgba(0xf59be63d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xfaa11dff).into(),
                        background: rgba(0xfaa11dff).into(),
                        selection: rgba(0xfaa11d3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xfe8080ff).into(),
                        background: rgba(0xfe8080ff).into(),
                        selection: rgba(0xfe80803d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x5beabcff).into(),
                        background: rgba(0x5beabcff).into(),
                        selection: rgba(0x5beabc3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xe35142ff).into(),
                        background: rgba(0xe35142ff).into(),
                        selection: rgba(0xe351423d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf1fe29ff).into(),
                        background: rgba(0xf1fe29ff).into(),
                        selection: rgba(0xf1fe293d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x5dea5aff).into(),
                        background: rgba(0x5dea5aff).into(),
                        selection: rgba(0x5dea5a3d).into(),
                    },
                ])),
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "embedded".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis.strong".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "enum".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf1fe29ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "hint".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x246e61ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "predictive".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x79434bff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "preproc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "primary".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.escape".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.regex".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special.symbol".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "text.literal".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5beabcff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
