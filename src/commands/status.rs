use anyhow::Result;
use console::style;
use serde_json::json;
use sysinfo::System;

use crate::core::size::format_size;
use crate::ui::progress;

pub fn run(json_output: bool) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Disk info
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let main_disk = disks.list().first();

    let disk_total = main_disk.map(|d| d.total_space()).unwrap_or(0);
    let disk_available = main_disk.map(|d| d.available_space()).unwrap_or(0);
    let disk_used = disk_total.saturating_sub(disk_available);

    // Memory
    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    // CPU
    let cpu_count = sys.cpus().len();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    // Top processes
    let mut processes: Vec<_> = sys
        .processes()
        .values()
        .map(|p| {
            (
                p.name().to_string_lossy().to_string(),
                p.memory(),
                p.cpu_usage(),
            )
        })
        .collect();

    // Sort by memory usage — collect top 5
    processes.sort_by(|a, b| b.1.cmp(&a.1));
    let top_mem: Vec<_> = processes.iter().take(5).cloned().collect();

    // Sort by CPU usage — collect top 5
    processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let top_cpu: Vec<_> = processes.iter().take(5).cloned().collect();

    // Health score (simple metric)
    let disk_usage_pct = if disk_total > 0 {
        (disk_used as f64 / disk_total as f64) * 100.0
    } else {
        0.0
    };
    let mem_usage_pct = if mem_total > 0 {
        (mem_used as f64 / mem_total as f64) * 100.0
    } else {
        0.0
    };

    let health_score = (100.0 - (disk_usage_pct * 0.5 + mem_usage_pct * 0.5)).max(0.0) as u32;

    if json_output {
        let output = json!({
            "disk": {
                "total": disk_total,
                "used": disk_used,
                "available": disk_available,
                "usage_percent": disk_usage_pct,
            },
            "memory": {
                "total": mem_total,
                "used": mem_used,
                "usage_percent": mem_usage_pct,
            },
            "swap": {
                "total": swap_total,
                "used": swap_used,
            },
            "cpu": {
                "brand": cpu_brand,
                "count": cpu_count,
            },
            "health_score": health_score,
            "top_processes_by_memory": top_mem.iter().map(|(name, mem, cpu)| {
                json!({"name": name, "memory": mem, "cpu_usage": *cpu})
            }).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("\n{}", style("  mop — System Status").bold().cyan());

    // Health Score
    let health_color = if health_score >= 80 {
        style(format!("{}%", health_score)).green().bold()
    } else if health_score >= 50 {
        style(format!("{}%", health_score)).yellow().bold()
    } else {
        style(format!("{}%", health_score)).red().bold()
    };
    println!("\n  Health Score: {}", health_color);

    // Disk
    progress::print_header("Disk Usage");
    print_usage_bar("Disk", disk_used, disk_total);

    // Memory
    progress::print_header("Memory");
    print_usage_bar("RAM", mem_used, mem_total);
    if swap_total > 0 {
        print_usage_bar("Swap", swap_used, swap_total);
    }

    // CPU
    progress::print_header("CPU");
    println!("  {} ({} cores)", style(&cpu_brand).bold(), cpu_count);

    // Top processes by memory
    progress::print_header("Top Processes (by Memory)");
    for (i, (name, mem, cpu)) in top_mem.iter().enumerate() {
        println!(
            "  {}. {:<30} RAM: {:<12} CPU: {:.1}%",
            i + 1,
            style(name).bold(),
            format_size(*mem),
            cpu
        );
    }

    // Top processes by CPU
    progress::print_header("Top Processes (by CPU)");
    for (i, (name, mem, cpu)) in top_cpu.iter().enumerate() {
        println!(
            "  {}. {:<30} CPU: {:<8.1}% RAM: {}",
            i + 1,
            style(name).bold(),
            cpu,
            format_size(*mem)
        );
    }

    Ok(())
}

fn print_usage_bar(label: &str, used: u64, total: u64) {
    let pct = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let bar_width = 30;
    let filled = (pct as usize * bar_width / 100).min(bar_width);
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_width - filled));

    let bar_colored = if pct >= 90 {
        style(&bar).red()
    } else if pct >= 70 {
        style(&bar).yellow()
    } else {
        style(&bar).green()
    };

    println!(
        "  {:<6} {} {} / {} ({}%)",
        label,
        bar_colored,
        format_size(used),
        format_size(total),
        pct
    );
}
