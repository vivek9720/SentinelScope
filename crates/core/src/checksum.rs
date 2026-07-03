pub fn ones_complement_sum(mut sum: u32, bytes: &[u8]) -> u32 {

    let mut chunks = bytes.chunks_exact(2);

    for chunk in &mut chunks {

        sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;

        while sum > 0xffff { sum = (sum & 0xffff) + (sum >> 16); }

    }

    if let Some(&last) = chunks.remainder().first() {

        sum += (last as u32) << 8;

        while sum > 0xffff { sum = (sum & 0xffff) + (sum >> 16); }

    }

    sum

}



pub fn internet_checksum(bytes: &[u8]) -> u16 { !(ones_complement_sum(0, bytes) as u16) }

pub fn verify_internet_checksum(bytes: &[u8]) -> bool { internet_checksum(bytes) == 0 }



pub fn ipv4_pseudo_header_sum(src: [u8; 4], dst: [u8; 4], protocol: u8, length: u16) -> u32 {

    let mut pseudo = [0u8; 12];

    pseudo[0..4].copy_from_slice(&src);

    pseudo[4..8].copy_from_slice(&dst);

    pseudo[9] = protocol;

    pseudo[10..12].copy_from_slice(&length.to_be_bytes());

    ones_complement_sum(0, &pseudo)

}



pub fn transport_checksum_ipv4(src: [u8; 4], dst: [u8; 4], protocol: u8, segment: &[u8]) -> u16 {

    let sum = ipv4_pseudo_header_sum(src, dst, protocol, segment.len() as u16);

    !(ones_complement_sum(sum, segment) as u16)

}



pub fn simple_fingerprint(bytes: &[u8]) -> u64 {

    let mut state: u64 = 0xcbf29ce484222325;

    for &b in bytes { state ^= b as u64; state = state.wrapping_mul(0x100000001b3); }

    state

}
