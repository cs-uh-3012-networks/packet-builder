use dioxus::prelude::*;

use crate::packet::{CtrFlags, PacketState, DATA_SIZE};

const PACKET_FORM_CSS: Asset = asset!("/assets/styling/packet_form.css");

#[component]
pub fn PacketForm(
    state: Signal<PacketState>,
    active_info: Signal<Option<&'static InfoContent>>,
    hovered_range: Signal<Option<(usize, usize)>>,
) -> Element {
    let mut active_info = active_info;
    let mut hovered_range = hovered_range;

    rsx! {
        document::Link { rel: "stylesheet", href: PACKET_FORM_CSS }

        div { class: "packet-form",
            h3 { "Packet Fields" }

            // --- IP Section ---
            fieldset { class: "layer-section layer-ip",
                legend { "IP Header" }
                div { class: "field-grid",
                    IpInput {
                        label: "Source IP",
                        initial: state.read().ip.src_ip,
                        on_valid: move |ip: [u8; 4]| { state.write().ip.src_ip = ip; },
                        on_focus: move |_| {
                            active_info.set(Some(&IP_INFO));
                            hovered_range.set(Some((26, 30)));
                        },
                        on_blur: move |_| { hovered_range.set(None); },
                    }
                    IpInput {
                        label: "Destination IP",
                        initial: state.read().ip.dst_ip,
                        on_valid: move |ip: [u8; 4]| { state.write().ip.dst_ip = ip; },
                        on_focus: move |_| {
                            active_info.set(Some(&IP_INFO));
                            hovered_range.set(Some((30, 34)));
                        },
                        on_blur: move |_| { hovered_range.set(None); },
                    }
                }
                div { class: "auto-fields",
                    "Other IP fields shown as XX (version, IHL, length, TTL, protocol, etc.)"
                }
            }

            // --- UDP Section ---
            fieldset { class: "layer-section layer-udp",
                legend { "UDP Header" }
                div { class: "field-grid",
                    FieldRow { label: "Source Port",
                        input {
                            r#type: "number",
                            min: 0,
                            max: 65535,
                            value: state.read().udp.src_port.to_string(),
                            oninput: move |e: Event<FormData>| {
                                if let Ok(v) = e.value().parse::<u16>() {
                                    state.write().udp.src_port = v;
                                }
                            },
                            onfocus: move |_| {
                                active_info.set(Some(&UDP_INFO));
                                hovered_range.set(Some((34, 36)));
                            },
                            onblur: move |_| { hovered_range.set(None); },
                        }
                    }
                    FieldRow { label: "Destination Port",
                        input {
                            r#type: "number",
                            min: 0,
                            max: 65535,
                            value: state.read().udp.dst_port.to_string(),
                            oninput: move |e: Event<FormData>| {
                                if let Ok(v) = e.value().parse::<u16>() {
                                    state.write().udp.dst_port = v;
                                }
                            },
                            onfocus: move |_| {
                                active_info.set(Some(&UDP_INFO));
                                hovered_range.set(Some((36, 38)));
                            },
                            onblur: move |_| { hovered_range.set(None); },
                        }
                    }
                }
                div { class: "auto-fields",
                    "Auto: length, checksum=0"
                }
            }

            // --- Custom TCP Section ---
            {
                let is_data = state.read().tcp.ctr_flags == CtrFlags::Data;
                rsx! {
                    fieldset { class: "layer-section layer-tcp",
                        legend { "Custom TCP Header" }
                        div { class: "field-grid",
                            FieldRow { label: "Sequence No.",
                                input {
                                    r#type: "number",
                                    min: 0,
                                    value: state.read().tcp.seqno.to_string(),
                                    disabled: !is_data,
                                    oninput: move |e: Event<FormData>| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            state.write().tcp.seqno = v;
                                        }
                                    },
                                    onfocus: move |_| {
                                        active_info.set(Some(&TCP_INFO));
                                        hovered_range.set(Some((42, 46)));
                                    },
                                    onblur: move |_| { hovered_range.set(None); },
                                }
                            }
                            FieldRow { label: "Ack No.",
                                input {
                                    r#type: "number",
                                    min: 0,
                                    value: state.read().tcp.ackno.to_string(),
                                    disabled: is_data,
                                    oninput: move |e: Event<FormData>| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            state.write().tcp.ackno = v;
                                        }
                                    },
                                    onfocus: move |_| {
                                        active_info.set(Some(&TCP_INFO));
                                        hovered_range.set(Some((46, 50)));
                                    },
                                    onblur: move |_| { hovered_range.set(None); },
                                }
                            }
                            FieldRow { label: "Flags",
                                select {
                                    value: match state.read().tcp.ctr_flags {
                                        CtrFlags::Data => "0",
                                        CtrFlags::Ack => "1",
                                    },
                                    oninput: move |e: Event<FormData>| {
                                        let mut s = state.write();
                                        match e.value().as_str() {
                                            "1" => {
                                                s.tcp.ctr_flags = CtrFlags::Ack;
                                                s.tcp.seqno = 0;
                                                s.payload.clear();
                                                s.tcp.data_size = 0;
                                            }
                                            _ => {
                                                s.tcp.ctr_flags = CtrFlags::Data;
                                                s.tcp.ackno = 0;
                                            }
                                        }
                                    },
                                    onfocus: move |_| {
                                        active_info.set(Some(&TCP_INFO));
                                        hovered_range.set(Some((50, 54)));
                                    },
                                    onblur: move |_| { hovered_range.set(None); },
                                    option { value: "0", "DATA (0)" }
                                    option { value: "1", "ACK (1)" }
                                }
                            }
                            FieldRow { label: "Data Size",
                                input {
                                    r#type: "number",
                                    disabled: true,
                                    value: state.read().tcp.data_size.to_string(),
                                }
                            }
                        }
                        div { class: "auto-fields",
                            "Big-endian byte order (network order). Data size tracks payload length."
                        }
                    }
                }
            }

            // --- Payload Section (hidden for ACK) ---
            if state.read().tcp.ctr_flags == CtrFlags::Data {
                fieldset { class: "layer-section layer-payload",
                    legend { "Payload" }
                    textarea {
                        class: "payload-input",
                        placeholder: "Enter payload text (max {DATA_SIZE} bytes)...",
                        maxlength: DATA_SIZE as i64,
                        rows: 4,
                        value: state.read().payload.clone(),
                        oninput: move |e: Event<FormData>| {
                            let mut s = state.write();
                            s.payload = e.value();
                            s.tcp.data_size = s.payload.len().min(DATA_SIZE) as u32;
                        },
                    }
                    div { class: "auto-fields",
                        "{state.read().payload.len()} / {DATA_SIZE} bytes"
                    }
                }
            }
        }
    }
}

// --- Info content for each layer (Wikipedia-style octet grid) ---

/// A cell in the header grid. `span` = how many octets (columns) it occupies.
/// `highlight` optionally overrides the hover byte range (relative to header start)
/// so that split fields (e.g. a 6-byte MAC across two rows) highlight as one unit.
#[derive(Clone, PartialEq)]
pub struct Cell {
    pub label: &'static str,
    pub span: usize,
    pub highlight: Option<(usize, usize)>,
}

/// A 4-octet row in the header diagram.
#[derive(Clone, PartialEq)]
pub struct Row {
    pub offset: usize,
    pub cells: &'static [Cell],
}

#[derive(Clone, PartialEq)]
pub struct InfoContent {
    pub title: &'static str,
    pub note: &'static str,
    pub rows: &'static [Row],
    pub base_offset: usize,
    pub css_class: &'static str,
}

pub const ETH_INFO: InfoContent = InfoContent {
    title: "Ethernet Header (14 bytes)",
    note: "All bytes shown as XX (not user-editable).",
    base_offset: 0,
    css_class: "info-layer-eth",
    rows: &[
        Row { offset: 0,  cells: &[Cell { label: "Destination MAC", span: 4, highlight: Some((0, 6)) }] },
        Row { offset: 4,  cells: &[Cell { label: "Destination MAC (cont.)", span: 2, highlight: Some((0, 6)) }, Cell { label: "Source MAC", span: 2, highlight: Some((6, 12)) }] },
        Row { offset: 8,  cells: &[Cell { label: "Source MAC (cont.)", span: 4, highlight: Some((6, 12)) }] },
        Row { offset: 12, cells: &[Cell { label: "EtherType", span: 2, highlight: None }] },
    ],
};

pub const IP_INFO: InfoContent = InfoContent {
    title: "IPv4 Header (20 bytes)",
    note: "Big-endian. Src/Dst IP editable, rest shown as XX.",
    base_offset: 14,
    css_class: "info-layer-ip",
    rows: &[
        Row { offset: 0,  cells: &[Cell { label: "Version+IHL", span: 1, highlight: None }, Cell { label: "DSCP+ECN", span: 1, highlight: None }, Cell { label: "Total Length", span: 2, highlight: None }] },
        Row { offset: 4,  cells: &[Cell { label: "Identification", span: 2, highlight: None }, Cell { label: "Flags+Frag Offset", span: 2, highlight: None }] },
        Row { offset: 8,  cells: &[Cell { label: "TTL", span: 1, highlight: None }, Cell { label: "Protocol", span: 1, highlight: None }, Cell { label: "Header Checksum", span: 2, highlight: None }] },
        Row { offset: 12, cells: &[Cell { label: "Source IP Address", span: 4, highlight: None }] },
        Row { offset: 16, cells: &[Cell { label: "Destination IP Address", span: 4, highlight: None }] },
    ],
};

pub const UDP_INFO: InfoContent = InfoContent {
    title: "UDP Header (8 bytes)",
    note: "Big-endian. Ports editable, length auto-computed, checksum=0.",
    base_offset: 34,
    css_class: "info-layer-udp",
    rows: &[
        Row { offset: 0, cells: &[Cell { label: "Source Port", span: 2, highlight: None }, Cell { label: "Destination Port", span: 2, highlight: None }] },
        Row { offset: 4, cells: &[Cell { label: "Length", span: 2, highlight: None }, Cell { label: "Checksum", span: 2, highlight: None }] },
    ],
};

pub const TCP_INFO: InfoContent = InfoContent {
    title: "Custom TCP Header (16 bytes)",
    note: "Big-endian (network order). Matches tcp_header_t C struct.",
    base_offset: 42,
    css_class: "info-layer-tcp",
    rows: &[
        Row { offset: 0,  cells: &[Cell { label: "seqno", span: 4, highlight: None }] },
        Row { offset: 4,  cells: &[Cell { label: "ackno", span: 4, highlight: None }] },
        Row { offset: 8,  cells: &[Cell { label: "ctr_flags", span: 4, highlight: None }] },
        Row { offset: 12, cells: &[Cell { label: "data_size", span: 4, highlight: None }] },
    ],
};

/// Given an absolute byte index, returns the matching InfoContent and the field's byte range.
pub fn info_for_byte(byte_idx: usize) -> Option<(&'static InfoContent, usize, usize)> {
    for info in [&ETH_INFO, &IP_INFO, &UDP_INFO, &TCP_INFO] {
        for row in info.rows {
            let mut col_offset = 0;
            for cell in row.cells {
                let cell_start = info.base_offset + row.offset + col_offset;
                let cell_end = cell_start + cell.span;
                if byte_idx >= cell_start && byte_idx < cell_end {
                    let (start, end) = match cell.highlight {
                        Some((s, e)) => (info.base_offset + s, info.base_offset + e),
                        None => (cell_start, cell_end),
                    };
                    return Some((info, start, end));
                }
                col_offset += cell.span;
            }
        }
    }
    None
}

// --- Reusable components ---

/// Helper component for a labeled form row.
#[component]
fn FieldRow(label: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "field-row",
            label { class: "field-label", "{label}" }
            div { class: "field-input", {children} }
        }
    }
}

/// IPv4 input with local text tracking and inline validation.
#[component]
fn IpInput(
    label: &'static str,
    initial: [u8; 4],
    on_valid: EventHandler<[u8; 4]>,
    on_focus: EventHandler,
    on_blur: EventHandler,
) -> Element {
    let mut text = use_signal(move || format_ip(&initial));
    let mut error = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "field-row",
            label { class: "field-label", "{label}" }
            div { class: "field-input",
                input {
                    r#type: "text",
                    class: if error.read().is_some() { "ip-invalid" } else { "" },
                    value: text(),
                    oninput: move |e: Event<FormData>| {
                        let val = e.value();
                        text.set(val.clone());
                        match validate_ip(&val) {
                            Ok(ip) => {
                                error.set(None);
                                on_valid.call(ip);
                            }
                            Err(msg) => {
                                error.set(Some(msg));
                            }
                        }
                    },
                    onfocus: move |_| { on_focus.call(()); },
                    onblur: move |_| { on_blur.call(()); },
                }
            }
            if let Some(msg) = error.read().as_ref() {
                span { class: "field-error", "{msg}" }
            }
        }
    }
}

fn format_ip(ip: &[u8; 4]) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}

/// Validate an IPv4 address string, returning the parsed octets or a helpful message.
fn validate_ip(s: &str) -> Result<[u8; 4], String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("Enter an IPv4 address".to_string());
    }

    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 {
        return Err(format!("Need 4 octets, got {}", parts.len()));
    }

    let mut ip = [0u8; 4];
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            return Err(format!("Octet {} is empty", i + 1));
        }
        match part.parse::<u16>() {
            Ok(v) if v <= 255 => ip[i] = v as u8,
            Ok(v) => return Err(format!("Octet {} is {} (max 255)", i + 1, v)),
            Err(_) => return Err(format!("Octet {} is not a number", i + 1)),
        }
    }
    Ok(ip)
}
