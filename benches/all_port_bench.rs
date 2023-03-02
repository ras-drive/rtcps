use criterion::{black_box, criterion_group, criterion_main, profiler::Profiler, Criterion};
use pprof::ProfilerGuard;
use rusty_port_scanner::port_scanner::PortScanner;
use std::{
    ffi::c_int,
    fs::File,
    net::{IpAddr, Ipv4Addr},
    path::Path,
};

pub struct FlamegraphProfiler<'a> {
    frequency: c_int,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl<'a> FlamegraphProfiler<'a> {
    #[allow(dead_code)]
    pub fn new(frequency: c_int) -> Self {
        FlamegraphProfiler {
            frequency,
            active_profiler: None,
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        self.active_profiler = Some(ProfilerGuard::new(self.frequency).unwrap());
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        std::fs::create_dir_all(benchmark_dir).unwrap();
        let flamegraph_path = benchmark_dir.join("flamegraph.svg");
        let flamegraph_file = File::create(&flamegraph_path)
            .expect("File system error while creating flamegraph.svg");
        if let Some(profiler) = self.active_profiler.take() {
            profiler
                .report()
                .build()
                .unwrap()
                .flamegraph(flamegraph_file)
                .expect("Error writing flamegraph");
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_all_ports_scanned", move |b| {
        b.to_async(criterion::async_executor::SmolExecutor)
            .iter(|| async { scan_ports().await })
    });
}

#[inline(always)]
async fn scan_ports() {
    let v: Vec<u16> = (0..=65535).collect();
    let mut port_scanner = PortScanner::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    port_scanner.scan_ports(black_box(v), None).await;
}

criterion_group! {name=benches; config=Criterion::default().sample_size(10).with_profiler(FlamegraphProfiler::new(100)); targets=criterion_benchmark}
criterion_main!(benches);
