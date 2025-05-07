tonic::include_proto!("psh.proto.instance");

impl From<std::net::Ipv6Addr> for Ipv6Addr {
    fn from(value: std::net::Ipv6Addr) -> Self {
        let ip = value.to_bits().to_be();
        let high = (ip >> 64) as u64;
        let low = ip as u64;
        Self {
            hi_64_bits: high,
            lo_64_bits: low,
        }
    }
}

impl From<Ipv6Addr> for std::net::Ipv6Addr {
    fn from(val: Ipv6Addr) -> Self {
        let ip = ((val.hi_64_bits as u128) << 64) | (val.lo_64_bits as u128);
        std::net::Ipv6Addr::from_bits(u128::from_be(ip))
    }
}

#[test]
fn test_ipv6_into_pb_repr() {
    use std::net::Ipv6Addr as StdIpv6Addr;

    let var: u128 = 1;

    let raw = StdIpv6Addr::from_bits(var);

    let pb_repr: Ipv6Addr = raw.into();

    let ip: StdIpv6Addr = pb_repr.into();

    assert_eq!(ip, StdIpv6Addr::from_bits(1));
}
