use perf_event_rs::sampling::record::{RecordBody, SampleId};

use self::perf_data_proto::{
    perf_event::EventType, AuxEvent, BpfEvent, CgroupEvent, CommEvent, ContextSwitchEvent,
    ForkEvent, ItraceStartEvent, KsymbolEvent, LostEvent, LostSamplesEvent, MMapEvent,
    NamespacesEvent, PerfEventAttr, ReadEvent, SampleEvent, SampleInfo, TextPokeEvent,
    ThrottleEvent, WeightStruct,
};

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

impl From<SampleId> for SampleInfo {
    fn from(value: SampleId) -> Self {
        Self {
            pid: value.pid,
            tid: value.tid,
            sample_time_ns: value.time,
            id: value.id_1,
            cpu: value.cpu,
            stream_id: value.stream_id,
        }
    }
}

impl From<perf_event_rs::sampling::record::mmap::Body> for MMapEvent {
    fn from(body: perf_event_rs::sampling::record::mmap::Body) -> Self {
        MMapEvent {
            pid: body.pid.into(),
            tid: body.tid.into(),
            start: body.addr.into(),
            len: body.len.into(),
            pgoff: body.pgoff.into(),
            maj: None,
            min: None,
            ino: None,
            ino_generation: None,
            build_id: None,
            prot: None,
            flags: None,
            filename: body.filename.into_string().ok(),
            filename_md5_prefix: None,
            root_path: None,
            root_path_md5_prefix: None,
            sample_info: None,
        }
    }
}

impl From<perf_event_rs::sampling::record::mmap2::Body> for MMapEvent {
    fn from(body: perf_event_rs::sampling::record::mmap2::Body) -> Self {
        let (maj, min, ino, ino_generation, build_id) = match body.anon_enum {
            perf_event_rs::sampling::record::mmap2::AnonEnum::Normal {
                maj,
                min,
                ino,
                ino_generation,
            } => (Some(maj), Some(min), Some(ino), Some(ino_generation), None),
            perf_event_rs::sampling::record::mmap2::AnonEnum::BuildId(items) => {
                let build_id = String::from_utf8(items).ok();
                (None, None, None, None, build_id)
            }
        };
        MMapEvent {
            pid: body.pid.into(),
            tid: body.tid.into(),
            start: body.addr.into(),
            len: body.len.into(),
            pgoff: body.pgoff.into(),
            maj,
            min,
            ino,
            ino_generation,
            build_id,
            prot: body.prot.into(),
            flags: body.flags.into(),
            filename: body.filename.into_string().ok(),
            filename_md5_prefix: None,
            root_path: None,
            root_path_md5_prefix: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::lost::Body> for LostEvent {
    fn from(body: perf_event_rs::sampling::record::lost::Body) -> Self {
        LostEvent {
            id: body.id.into(),
            lost: body.lost.into(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::comm::Body> for CommEvent {
    fn from(body: perf_event_rs::sampling::record::comm::Body) -> Self {
        CommEvent {
            pid: body.pid.into(),
            tid: body.tid.into(),
            comm: body.comm.into_string().ok(),
            comm_md5_prefix: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::exit::Body> for ForkEvent {
    fn from(body: perf_event_rs::sampling::record::exit::Body) -> Self {
        ForkEvent {
            pid: body.pid.into(),
            ppid: body.ppid.into(),
            tid: body.tid.into(),
            ptid: body.ptid.into(),
            fork_time_ns: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::throttle::Body> for ThrottleEvent {
    fn from(body: perf_event_rs::sampling::record::throttle::Body) -> Self {
        ThrottleEvent {
            time_ns: body.time.into(),
            id: body.id.into(),
            stream_id: body.stream_id.into(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::fork::Body> for ForkEvent {
    fn from(body: perf_event_rs::sampling::record::fork::Body) -> Self {
        Self {
            pid: body.pid.into(),
            ppid: body.ppid.into(),
            tid: body.tid.into(),
            ptid: body.ptid.into(),
            fork_time_ns: body.time.into(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}

impl From<perf_event_rs::sampling::record::read::Body> for ReadEvent {
    fn from(body: perf_event_rs::sampling::record::read::Body) -> Self {
        Self {
            pid: body.pid.into(),
            tid: body.tid.into(),
            value: None,
            time_enabled: body.values.time_enabled.into(),
            time_running: body.values.time_running.into(),
            id: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::sample::Body> for SampleEvent {
    fn from(body: perf_event_rs::sampling::record::sample::Body) -> Self {
        let (weight, weight_struct) = match body.weight {
            Some(weight) => match weight {
                perf_event_rs::sampling::record::sample::Weight::Full(f) => (Some(f), None),
                perf_event_rs::sampling::record::sample::Weight::Vars {
                    var1_dw,
                    var2_w,
                    var3_w,
                } => (
                    None,
                    Some(WeightStruct {
                        var1_dw: var1_dw.into(),
                        var2_w: (var2_w as u32).into(),
                        var3_w: (var3_w as u32).into(),
                    }),
                ),
            },
            None => (None, None),
        };
        Self {
            ip: body.ip,
            pid: body.pid,
            tid: body.tid,
            sample_time_ns: body.time,
            addr: body.addr,
            id: body.id,
            stream_id: body.stream_id,
            period: body.period,
            cpu: body.cpu,
            raw_size: body.data_raw.as_ref().map(|v| v.len() as _),
            raw: body.data_raw,
            read_info: None,
            callchain: body.ips.unwrap_or_default(),
            branch_stack: vec![],
            weight,
            weight_struct,
            data_src: None,
            transaction: None,
            physical_addr: body.phys_addr,
            cgroup: body.cgroup,
            data_page_size: body.data_page_size,
            code_page_size: body.code_page_size,
            no_hw_idx: None,
            branch_stack_hw_idx: None,
        }
    }
}

impl From<perf_event_rs::sampling::record::aux::Body> for AuxEvent {
    fn from(body: perf_event_rs::sampling::record::aux::Body) -> Self {
        Self {
            aux_offset: body.aux_offset.into(),
            aux_size: body.aux_size.into(),
            is_truncated: None,
            is_overwrite: None,
            is_partial: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::intrace_start::Body> for ItraceStartEvent {
    fn from(body: perf_event_rs::sampling::record::intrace_start::Body) -> Self {
        Self {
            pid: body.pid.into(),
            tid: body.tid.into(),
            sample_info: None,
        }
    }
}
impl From<perf_event_rs::sampling::record::lost_samples::Body> for LostSamplesEvent {
    fn from(body: perf_event_rs::sampling::record::lost_samples::Body) -> Self {
        Self {
            num_lost: body.lost.into(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::switch::Body> for ContextSwitchEvent {
    fn from(body: perf_event_rs::sampling::record::switch::Body) -> Self {
        Self {
            is_out: None,
            next_prev_pid: None,
            next_prev_tid: None,
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::switch_cpu_wide::Body> for ContextSwitchEvent {
    fn from(body: perf_event_rs::sampling::record::switch_cpu_wide::Body) -> Self {
        Self {
            is_out: None,
            next_prev_pid: body.next_prev_pid.into(),
            next_prev_tid: body.next_prev_tid.into(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::namespaces::Body> for NamespacesEvent {
    fn from(body: perf_event_rs::sampling::record::namespaces::Body) -> Self {
        Self {
            pid: body.pid.into(),
            tid: body.tid.into(),
            link_info: vec![],
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::ksymbol::Body> for KsymbolEvent {
    fn from(body: perf_event_rs::sampling::record::ksymbol::Body) -> Self {
        Self {
            addr: body.addr.into(),
            len: body.len.into(),
            ksym_type: (body.ksym_type as u32).into(),
            flags: (body.flags as u32).into(),
            name: body.name.into_string().ok(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::bpf_event::Body> for BpfEvent {
    fn from(body: perf_event_rs::sampling::record::bpf_event::Body) -> Self {
        Self {
            r#type: body.r#type as _,
            flags: body.flags as _,
            id: body.id,
            tag: body.tag.to_vec(),
            sample_info: body.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::cgroup::Body> for CgroupEvent {
    fn from(value: perf_event_rs::sampling::record::cgroup::Body) -> Self {
        Self {
            id: value.id.into(),
            path: value.path.into_string().ok(),
            sample_info: value.sample_id.map(Into::into),
        }
    }
}
impl From<perf_event_rs::sampling::record::text_poke::Body> for TextPokeEvent {
    fn from(value: perf_event_rs::sampling::record::text_poke::Body) -> Self {
        Self {
            addr: value.addr,
            old_len: value.old_len as _,
            new_len: value.new_len as _,
            bytes: value.bytes,
            sample_info: value.sample_id.map(Into::into),
        }
    }
}

impl From<RecordBody> for EventType {
    fn from(value: RecordBody) -> Self {
        match value {
            RecordBody::Mmap(body) => Self::MmapEvent((*body).into()),
            RecordBody::Lost(body) => Self::LostEvent((*body).into()),
            RecordBody::Comm(body) => Self::CommEvent((*body).into()),
            RecordBody::Exit(body) => Self::ExitEvent((*body).into()),
            RecordBody::Throttle(body) => Self::ThrottleEvent((*body).into()),
            RecordBody::Unthrottle(body) => Self::ThrottleEvent((*body).into()),
            RecordBody::Fork(body) => Self::ForkEvent((*body).into()),
            RecordBody::Read(body) => Self::ReadEvent((*body).into()),
            RecordBody::Sample(body) => Self::SampleEvent((*body).into()),
            RecordBody::Mmap2(body) => Self::MmapEvent((*body).into()),
            RecordBody::Aux(body) => Self::AuxEvent((*body).into()),
            RecordBody::ItraceStart(body) => Self::ItraceStartEvent((*body).into()),
            RecordBody::LostSamples(body) => Self::LostSamplesEvent((*body).into()),
            RecordBody::Switch(body) => Self::ContextSwitchEvent((*body).into()),
            RecordBody::SwitchCpuWide(body) => Self::ContextSwitchEvent((*body).into()),
            RecordBody::Namespaces(body) => Self::NamespacesEvent((*body).into()),
            RecordBody::Ksymbol(body) => Self::KsymbolEvent((*body).into()),
            RecordBody::BpfEvent(body) => Self::BpfEvent((*body).into()),
            RecordBody::Cgroup(body) => Self::CgroupEvent((*body).into()),
            RecordBody::TextPoke(body) => Self::TextPokeEvent((*body).into()),
            RecordBody::AuxOutputHwId(_body) => todo!(),
        }
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
