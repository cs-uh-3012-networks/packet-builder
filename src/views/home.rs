use dioxus::prelude::*;

use crate::components::{HexView, InfoContent, PacketForm, info_for_byte};
use crate::packet::PacketState;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let state = use_signal(PacketState::default);
    let mut active_info: Signal<Option<&'static InfoContent>> = use_signal(|| None);
    let mut hovered_range: Signal<Option<(usize, usize)>> = use_signal(|| None);
    let annotated = state.read().to_annotated_bytes();

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { class: "home-layout",
            PacketForm { state, active_info, hovered_range }
            div { class: "right-column",
                HexView {
                    annotated,
                    hovered_range,
                    on_byte_hover: move |idx: Option<usize>| {
                        match idx {
                            Some(i) => {
                                if let Some((info, start, end)) = info_for_byte(i) {
                                    active_info.set(Some(info));
                                    hovered_range.set(Some((start, end)));
                                }
                            }
                            None => {
                                hovered_range.set(None);
                            }
                        }
                    },
                }
                if let Some(info) = *active_info.read() {
                    div { class: "info-panel {info.css_class}",
                        div { class: "info-panel-header",
                            span { class: "info-panel-title", "{info.title}" }
                            button {
                                class: "info-panel-close",
                                onclick: move |_| { active_info.set(None); },
                                "x"
                            }
                        }
                        table { class: "info-table",
                            thead {
                                tr {
                                    th { class: "info-offset-col", "Offset" }
                                    th { "Octet 0" }
                                    th { "Octet 1" }
                                    th { "Octet 2" }
                                    th { "Octet 3" }
                                }
                            }
                            tbody {
                                for row in info.rows {
                                    tr {
                                        td { class: "info-offset", "{row.offset}" }
                                        {
                                            let mut col_offset = 0usize;
                                            rsx! {
                                                for cell in row.cells {
                                                    {
                                                        let cell_start = info.base_offset + row.offset + col_offset;
                                                        let cell_end = cell_start + cell.span;
                                                        col_offset += cell.span;
                                                        let (hl_start, hl_end) = match cell.highlight {
                                                            Some((s, e)) => (info.base_offset + s, info.base_offset + e),
                                                            None => (cell_start, cell_end),
                                                        };
                                                        {
                                                            let is_active = matches!(*hovered_range.read(), Some((s, e)) if s == hl_start && e == hl_end);
                                                            let class = if is_active { "info-cell info-cell-highlight" } else { "info-cell" };
                                                            rsx! {
                                                                td {
                                                                    colspan: "{cell.span}",
                                                                    class,
                                                                    onmouseenter: move |_| { hovered_range.set(Some((hl_start, hl_end))); },
                                                                    onmouseleave: move |_| { hovered_range.set(None); },
                                                                    "{cell.label}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "info-note", "{info.note}" }
                    }
                }
            }
        }
    }
}
