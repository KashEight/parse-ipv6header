use std::{env, net::Ipv6Addr, process, u16};

struct V6Header {
    version: u8,
    traffic_class: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: u8,
    hop_limit: u8,
    source_address: Ipv6Addr,
    destination_address: Ipv6Addr,
}

impl V6Header {
    fn new(header: &str) -> V6Header {
        let radix = 16;

        let four_octet = u32::from_str_radix(&header[..8], radix).unwrap();
        let version = four_octet >> 28; // 4bit
        let traffic_class = (four_octet >> 20) & 255; // 8bit
        let flow_label = four_octet & 1048575; // 20bit

        let four_octet = u32::from_str_radix(&header[8..16], radix).unwrap();
        let payload_length = four_octet >> 16; // 16bit
        let next_header = (four_octet >> 8) & 255; // 8bit
        let hop_limit = four_octet & 255; // 8bit

        let four_octet = u32::from_str_radix(&header[16..24], radix).unwrap();
        let sa = (four_octet >> 16) as u16;
        let sb = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[24..32], radix).unwrap();
        let sc = (four_octet >> 16) as u16;
        let sd = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[32..40], radix).unwrap();
        let se = (four_octet >> 16) as u16;
        let sf = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[40..48], radix).unwrap();
        let sg = (four_octet >> 16) as u16;
        let sh = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[48..56], radix).unwrap();
        let da = (four_octet >> 16) as u16;
        let db = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[56..64], radix).unwrap();
        let dc = (four_octet >> 16) as u16;
        let dd = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[64..72], radix).unwrap();
        let de = (four_octet >> 16) as u16;
        let df = (four_octet & 0xFFFF) as u16;

        let four_octet = u32::from_str_radix(&header[72..80], radix).unwrap();
        let dg = (four_octet >> 16) as u16;
        let dh = (four_octet & 0xFFFF) as u16;

        let v6header = V6Header {
            version: version as u8,
            traffic_class: traffic_class as u8,
            flow_label: flow_label as u32,
            payload_length: payload_length as u16,
            next_header: next_header as u8,
            hop_limit: hop_limit as u8,
            source_address: Ipv6Addr::new(sa, sb, sc, sd, se, sf, sg, sh),
            destination_address: Ipv6Addr::new(da, db, dc, dd, de, df, dg, dh)
        };

        return v6header;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("too many arguments!");
        process::exit(1);
    } else if args.len() < 2 {
        eprintln!("No argument!");
        process::exit(1)
    }

    let header = V6Header::new(&args[1][..80]);

    println!("version: {}", header.version);
    println!("traffic_class: {}", header.traffic_class);
    println!("flow label: {}", header.flow_label);
    println!("payload length: {}", header.payload_length);
    println!("next header: {}", header.next_header);
    println!("hop limit: {}", header.hop_limit);
    println!("source address: {}", header.source_address.to_string());
    println!("destination address: {}", header.destination_address.to_string());
}
