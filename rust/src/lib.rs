use self::perf_data_proto::PerfEventAttr;

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

impl From<perf_event_rs::PerfEventAttr> for PerfEventAttr {
    fn from(attr: perf_event_rs::PerfEventAttr) -> Self {
        attr.0.into()
    }
}

impl From<perf_event_rs::RawPerfEventAttr> for PerfEventAttr {
    fn from(attr: perf_event_rs::RawPerfEventAttr) -> Self {
        let watermark = attr.watermark() != 0;

        let (wakeup_watermark, wakeup_events) = if watermark {
            (
                unsafe { attr.__bindgen_anon_2.wakeup_watermark.into() },
                None,
            )
        } else {
            (None, unsafe { attr.__bindgen_anon_2.wakeup_events.into() })
        };

        let freq = attr.freq() != 0;

        let (sample_period, sample_freq) = if freq {
            (unsafe { attr.__bindgen_anon_1.sample_freq.into() }, None)
        } else {
            (None, unsafe { attr.__bindgen_anon_1.sample_period.into() })
        };

        PerfEventAttr {
            r#type: attr.type_.into(),
            size: attr.size.into(),
            config: attr.config.into(),
            sample_period,
            sample_freq,
            sample_type: attr.sample_type.into(),
            read_format: attr.read_format.into(),
            disabled: Some(attr.disabled() != 0),
            inherit: Some(attr.inherit() != 0),
            pinned: Some(attr.pinned() != 0),
            exclusive: Some(attr.exclusive() != 0),
            exclude_user: Some(attr.exclude_user() != 0),
            exclude_kernel: Some(attr.exclude_kernel() != 0),
            exclude_hv: Some(attr.exclude_hv() != 0),
            exclude_idle: Some(attr.exclude_idle() != 0),
            mmap: Some(attr.mmap() != 0),
            comm: Some(attr.comm() != 0),
            freq: Some(freq),
            inherit_stat: Some(attr.inherit_stat() != 0),
            enable_on_exec: Some(attr.enable_on_exec() != 0),
            task: Some(attr.task() != 0),
            watermark: Some(watermark),
            precise_ip: Some(attr.precise_ip() as _),
            mmap_data: Some(attr.mmap_data() != 0),
            sample_id_all: Some(attr.sample_id_all() != 0),
            exclude_host: Some(attr.exclude_host() != 0),
            exclude_guest: Some(attr.exclude_guest() != 0),
            exclude_callchain_kernel: Some(attr.exclude_callchain_kernel() != 0),
            exclude_callchain_user: Some(attr.exclude_callchain_user() != 0),
            mmap2: Some(attr.mmap2() != 0),
            comm_exec: Some(attr.comm_exec() != 0),
            use_clockid: Some(attr.use_clockid() != 0),
            context_switch: Some(attr.context_switch() != 0),
            write_backward: Some(attr.write_backward() != 0),
            namespaces: Some(attr.namespaces() != 0),
            cgroup: Some(attr.cgroup() != 0),
            ksymbol: Some(attr.ksymbol() != 0),
            wakeup_events,
            wakeup_watermark,
            bp_type: attr.bp_type.into(),
            bp_addr: unsafe { attr.__bindgen_anon_3.bp_addr.into() },
            config1: attr.config.into(),
            bp_len: unsafe { attr.__bindgen_anon_4.bp_len.into() },
            config2: attr.config3.into(),
            branch_sample_type: attr.branch_sample_type.into(),
            sample_regs_user: attr.sample_regs_user.into(),
            sample_stack_user: attr.sample_stack_user.into(),
        }
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
