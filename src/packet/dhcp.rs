use std::net::Ipv4Addr;
use crate::datalink::MacAddr;

/// Represents an Dhcp operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DhcpOperation{
    Request = 1,
    Reply = 2,
}

impl DhcpOperation {
    pub fn from_u8(n: u8) -> Option<DhcpOperation> {
        match n {
            1 => Some(DhcpOperation::Request),
            2 => Some(DhcpOperation::Reply),
            _ => None,
        }
    }
}

/// Represents the Dhcp hardware types.
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DhcpHardwareType{
    Ethernet = 1,
    ExperimentalEthernet = 2,
    AmateurRadioAX25 = 3,
    ProteonProNETTokenRing = 4,
    Chaos = 5,
    IEEE802Networks = 6,
    ARCNET = 7,
    Hyperchannel = 8,
    Lanstar = 9,
    AutonetShortAddress = 10,
    LocalTalk = 11,
    LocalNet = 12,
    UltraLink = 13,
    SMDS = 14,
    FrameRelay = 15,
    AsynchronousTransmissionMode = 16,
    HDLC = 17,
    FibreChannel = 18,
    AsynchronousTransmissionMode1 = 19,
    PropPointToPointSerial = 20,
    PPP = 21,
    SoftwareLoopback = 24,
    EON = 25,
    Ethernet3MB = 26,
    NSIP = 27,
    Slip = 28,
    ULTRALink = 29,
    DS3 = 30,
    SIP = 31,
    FrameRelayInterconnect = 32,
    AsynchronousTransmissionMode2 = 33,
    MILSTD188220 = 34,
    Metricom = 35,
    IEEE1394 = 37,
    MAPOS = 39,
    Twinaxial = 40,
    EUI64 = 41,
    HIPARP = 42,
    IPandARPoverISO7816_3 = 43,
    ARPSec = 44,
    IPsecTunnel = 45,
    InfiniBand = 47,
    TIA102Project25CommonAirInterface = 48,
    WiegandInterface = 49,
    PureIP = 50,
    HWExp1 = 51,
    HFI = 52,
    HWExp2 = 53,
    AEthernet = 54,
    HWExp3 = 55,
    IPsecTransport = 56,
    SDLCRadio = 57,
    SDLCMultipoint = 58,
    IWARP = 59,
    SixLoWPAN = 61,
    VLAN = 62,
    ProviderBridging = 63,
    IEEE802154 = 64,
    MAPOSinIPv4 = 65,
    MAPOSinIPv6 = 66,
    IEEE802154NonASKPHY = 70,
}

impl DhcpHardwareType {
    pub fn from_u8(n: u8) -> Option<DhcpHardwareType> {
        match n {
            1 => Some(DhcpHardwareType::Ethernet),
            2 => Some(DhcpHardwareType::ExperimentalEthernet),
            3 => Some(DhcpHardwareType::AmateurRadioAX25),
            4 => Some(DhcpHardwareType::ProteonProNETTokenRing),
            5 => Some(DhcpHardwareType::Chaos),
            6 => Some(DhcpHardwareType::IEEE802Networks),
            7 => Some(DhcpHardwareType::ARCNET),
            8 => Some(DhcpHardwareType::Hyperchannel),
            9 => Some(DhcpHardwareType::Lanstar),
            10 => Some(DhcpHardwareType::AutonetShortAddress),
            11 => Some(DhcpHardwareType::LocalTalk),
            12 => Some(DhcpHardwareType::LocalNet),
            13 => Some(DhcpHardwareType::UltraLink),
            14 => Some(DhcpHardwareType::SMDS),
            15 => Some(DhcpHardwareType::FrameRelay),
            16 => Some(DhcpHardwareType::AsynchronousTransmissionMode),
            17 => Some(DhcpHardwareType::HDLC),
            18 => Some(DhcpHardwareType::FibreChannel),
            19 => Some(DhcpHardwareType::AsynchronousTransmissionMode1),
            20 => Some(DhcpHardwareType::PropPointToPointSerial),
            21 => Some(DhcpHardwareType::PPP),
            24 => Some(DhcpHardwareType::SoftwareLoopback),
            25 => Some(DhcpHardwareType::EON),
            26 => Some(DhcpHardwareType::Ethernet3MB),
            27 => Some(DhcpHardwareType::NSIP),
            28 => Some(DhcpHardwareType::Slip),
            29 => Some(DhcpHardwareType::ULTRALink),
            30 => Some(DhcpHardwareType::DS3),
            31 => Some(DhcpHardwareType::SIP),
            32 => Some(DhcpHardwareType::FrameRelayInterconnect),
            33 => Some(DhcpHardwareType::AsynchronousTransmissionMode2),
            34 => Some(DhcpHardwareType::MILSTD188220),
            35 => Some(DhcpHardwareType::Metricom),
            37 => Some(DhcpHardwareType::IEEE1394),
            39 => Some(DhcpHardwareType::MAPOS),
            40 => Some(DhcpHardwareType::Twinaxial),
            41 => Some(DhcpHardwareType::EUI64),
            42 => Some(DhcpHardwareType::HIPARP),
            43 => Some(DhcpHardwareType::IPandARPoverISO7816_3),
            44 => Some(DhcpHardwareType::ARPSec),
            45 => Some(DhcpHardwareType::IPsecTunnel),
            47 => Some(DhcpHardwareType::InfiniBand),
            48 => Some(DhcpHardwareType::TIA102Project25CommonAirInterface),
            49 => Some(DhcpHardwareType::WiegandInterface),
            50 => Some(DhcpHardwareType::PureIP),
            51 => Some(DhcpHardwareType::HWExp1),
            52 => Some(DhcpHardwareType::HFI),
            53 => Some(DhcpHardwareType::HWExp2),
            54 => Some(DhcpHardwareType::AEthernet),
            55 => Some(DhcpHardwareType::HWExp3),
            56 => Some(DhcpHardwareType::IPsecTransport),
            57 => Some(DhcpHardwareType::SDLCRadio),
            58 => Some(DhcpHardwareType::SDLCMultipoint),
            59 => Some(DhcpHardwareType::IWARP),
            61 => Some(DhcpHardwareType::SixLoWPAN),
            62 => Some(DhcpHardwareType::VLAN),
            63 => Some(DhcpHardwareType::ProviderBridging),
            64 => Some(DhcpHardwareType::IEEE802154),
            65 => Some(DhcpHardwareType::MAPOSinIPv4),
            66 => Some(DhcpHardwareType::MAPOSinIPv6),
            70 => Some(DhcpHardwareType::IEEE802154NonASKPHY),
            _ => None,
        }
    }
}

/// Represents an DHCP Packet.
#[derive(Clone, Debug, PartialEq)]
pub struct DhcpPacket {
    pub op: DhcpOperation,
    pub htype: DhcpHardwareType,
    pub hlen: u8,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: Ipv4Addr,
    pub yiaddr: Ipv4Addr,
    pub siaddr: Ipv4Addr,
    pub giaddr: Ipv4Addr,
    pub chaddr: MacAddr,
    pub chaddr_pad: Vec<u8>,
    pub sname: Vec<u8>,
    pub file: Vec<u8>,
    pub options: Vec<u8>,
}
