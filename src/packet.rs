/// Constants matching the C defines in the networking course.
pub const MSS_SIZE: usize = 1500;
pub const ETH_HDR_SIZE: usize = 14;
pub const IP_HDR_SIZE: usize = 20;
pub const UDP_HDR_SIZE: usize = 8;
pub const TCP_HDR_SIZE: usize = 16;
pub const DATA_SIZE: usize = 1456;

/// Which protocol layer a byte belongs to, used for color-coding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layer {
    Ethernet,
    Ip,
    Udp,
    Tcp,
    Payload,
}

impl Layer {
    pub fn color(&self) -> &'static str {
        match self {
            Layer::Ethernet => "#f5a623",
            Layer::Ip => "#4a90e2",
            Layer::Udp => "#50c878",
            Layer::Tcp => "#e26b4a",
            Layer::Payload => "#9b59b6",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Layer::Ethernet => "Ethernet",
            Layer::Ip => "IP",
            Layer::Udp => "UDP",
            Layer::Tcp => "TCP",
            Layer::Payload => "Payload",
        }
    }
}

/// IP header fields — only source and destination IP are user-editable.
#[derive(Debug, Clone, PartialEq)]
pub struct IpFields {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
}

impl Default for IpFields {
    fn default() -> Self {
        Self {
            src_ip: [10, 0, 0, 1],
            dst_ip: [10, 0, 0, 2],
        }
    }
}

/// UDP header fields (editable subset).
#[derive(Debug, Clone, PartialEq)]
pub struct UdpFields {
    pub src_port: u16,
    pub dst_port: u16,
}

impl Default for UdpFields {
    fn default() -> Self {
        Self {
            src_port: 5000,
            dst_port: 5001,
        }
    }
}

/// Control flags for the custom TCP header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CtrFlags {
    Data = 0,
    Ack = 1,
}

/// Custom TCP header fields (matches tcp_header_t C struct).
/// Uses big-endian (network byte order).
#[derive(Debug, Clone, PartialEq)]
pub struct TcpFields {
    pub seqno: u32,
    pub ackno: u32,
    pub ctr_flags: CtrFlags,
    pub data_size: u32,
}

impl Default for TcpFields {
    fn default() -> Self {
        Self {
            seqno: 0,
            ackno: 0,
            ctr_flags: CtrFlags::Data,
            data_size: 0,
        }
    }
}

/// Full packet state owned by the Home component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PacketState {
    pub ip: IpFields,
    pub udp: UdpFields,
    pub tcp: TcpFields,
    pub payload: String,
}

/// A single annotated byte in the packet.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedByte {
    pub value: u8,
    pub layer: Layer,
    /// If true, this byte is not user-specified and should display as "XX".
    pub placeholder: bool,
}

impl PacketState {
    /// Serialize the full packet to annotated bytes.
    /// Ethernet bytes and non-editable IP fields are marked as placeholders (XX).
    /// IP and UDP headers use big-endian (network byte order).
    /// The custom TCP struct uses big-endian (network byte order).
    pub fn to_annotated_bytes(&self) -> Vec<AnnotatedByte> {
        let payload_bytes = self.payload.as_bytes();
        let payload_len = payload_bytes.len().min(DATA_SIZE);
        let total_len = ETH_HDR_SIZE + IP_HDR_SIZE + UDP_HDR_SIZE + TCP_HDR_SIZE + payload_len;

        let mut out = Vec::with_capacity(total_len);

        // --- Ethernet Header (14 bytes, all placeholder) ---
        // Destination MAC (6) + Source MAC (6) + EtherType (2)
        out.extend(std::iter::repeat_n(AnnotatedByte { value: 0x00, layer: Layer::Ethernet, placeholder: true }, 14));

        // --- IP Header (20 bytes, big-endian) ---
        // Bytes 0-11: version, DSCP, total length, identification, flags, TTL, protocol, checksum — all XX
        let ip_total_len = (IP_HDR_SIZE + UDP_HDR_SIZE + TCP_HDR_SIZE + payload_len) as u16;
        for &b in &[0x45u8, 0x00] {
            out.push(AnnotatedByte { value: b, layer: Layer::Ip, placeholder: true });
        }
        for &b in &ip_total_len.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Ip, placeholder: true });
        }
        // Identification + Flags/FragOffset + TTL + Protocol + Checksum (8 bytes)
        for &b in &[0x00, 0x00, 0x00, 0x00, 0x40, 17, 0x00, 0x00] {
            out.push(AnnotatedByte { value: b, layer: Layer::Ip, placeholder: true });
        }
        // Source IP — user-specified
        for &b in &self.ip.src_ip {
            out.push(AnnotatedByte { value: b, layer: Layer::Ip, placeholder: false });
        }
        // Destination IP — user-specified
        for &b in &self.ip.dst_ip {
            out.push(AnnotatedByte { value: b, layer: Layer::Ip, placeholder: false });
        }

        // --- UDP Header (8 bytes, big-endian) ---
        let udp_len = (UDP_HDR_SIZE + TCP_HDR_SIZE + payload_len) as u16;
        for &b in &self.udp.src_port.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Udp, placeholder: false });
        }
        for &b in &self.udp.dst_port.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Udp, placeholder: false });
        }
        for &b in &udp_len.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Udp, placeholder: false });
        }
        for &b in &0u16.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Udp, placeholder: true });
        }

        // --- Custom TCP Header (16 bytes, big-endian / network order) ---
        for &b in &self.tcp.seqno.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Tcp, placeholder: false });
        }
        for &b in &self.tcp.ackno.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Tcp, placeholder: false });
        }
        for &b in &(self.tcp.ctr_flags as u32).to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Tcp, placeholder: false });
        }
        for &b in &self.tcp.data_size.to_be_bytes() {
            out.push(AnnotatedByte { value: b, layer: Layer::Tcp, placeholder: false });
        }

        // --- Payload ---
        for &b in &payload_bytes[..payload_len] {
            out.push(AnnotatedByte { value: b, layer: Layer::Payload, placeholder: false });
        }

        out
    }
}
