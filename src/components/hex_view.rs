use dioxus::prelude::*;

use crate::packet::{AnnotatedByte, Layer, ETH_HDR_SIZE, IP_HDR_SIZE, TCP_HDR_SIZE, UDP_HDR_SIZE};

const HEX_VIEW_CSS: Asset = asset!("/assets/styling/hex_view.css");

const BYTES_PER_ROW: usize = 16;

#[component]
pub fn HexView(annotated: Vec<AnnotatedByte>, hovered_range: Signal<Option<(usize, usize)>>, on_byte_hover: EventHandler<Option<usize>>) -> Element {
    let rows: Vec<_> = annotated.chunks(BYTES_PER_ROW).enumerate().collect();

    rsx! {
        document::Link { rel: "stylesheet", href: HEX_VIEW_CSS }

        div { class: "hex-view",
            h3 { "Hex Dump" }

            // Legend
            div { class: "hex-legend",
                for layer in [Layer::Ethernet, Layer::Ip, Layer::Udp, Layer::Tcp, Layer::Payload] {
                    span {
                        class: "legend-item",
                        span {
                            class: "legend-swatch",
                            style: "background-color: {layer.color()};",
                        }
                        "{layer.label()}"
                    }
                }
            }

            div { class: "hex-dump",
                // Header row
                div { class: "hex-row hex-header",
                    span { class: "hex-offset", "Offset" }
                    span { class: "hex-bytes",
                        for i in 0..BYTES_PER_ROW {
                            span { class: "hex-byte", "{i:02X}" }
                        }
                    }
                }

                if annotated.is_empty() {
                    div { class: "hex-row hex-empty",
                        span { class: "hex-offset", "—" }
                        span { class: "hex-bytes", "No data. Fill in fields above." }
                    }
                }

                for (row_idx, chunk) in rows {
                    {
                        let offset = row_idx * BYTES_PER_ROW;
                        let hr = *hovered_range.read();
                        rsx! {
                            div { class: "hex-row",
                                span { class: "hex-offset", "{offset:04X}" }
                                span { class: "hex-bytes",
                                    for (pos, ab) in chunk.iter().enumerate() {
                                        {
                                            let abs_idx = offset + pos;
                                            let highlighted = matches!(hr, Some((start, end)) if abs_idx >= start && abs_idx < end);
                                            let class = if highlighted { "hex-byte hex-byte-highlight" } else { "hex-byte" };
                                            rsx! {
                                                span {
                                                    class,
                                                    style: "color: {ab.layer.color()};",
                                                    onmouseenter: move |_| { on_byte_hover.call(Some(abs_idx)); },
                                                    onmouseleave: move |_| { on_byte_hover.call(None); },
                                                    {if ab.placeholder { "XX".to_string() } else { format!("{:02X}", ab.value) }}
                                                }
                                            }
                                        }
                                    }
                                    // Pad short last row
                                    if chunk.len() < BYTES_PER_ROW {
                                        for _ in 0..(BYTES_PER_ROW - chunk.len()) {
                                            span { class: "hex-byte hex-pad", "  " }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Byte count summary
            div { class: "hex-summary",
                {format!(
                    "Total: {} bytes (Eth: {} + IP: {} + UDP: {} + TCP: {} + Payload: {})",
                    annotated.len(),
                    ETH_HDR_SIZE,
                    IP_HDR_SIZE,
                    UDP_HDR_SIZE,
                    TCP_HDR_SIZE,
                    annotated.len().saturating_sub(ETH_HDR_SIZE + IP_HDR_SIZE + UDP_HDR_SIZE + TCP_HDR_SIZE),
                )}
            }
        }
    }
}

