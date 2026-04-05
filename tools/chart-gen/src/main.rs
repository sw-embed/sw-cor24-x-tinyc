//! chart-gen: Extract test counts from git history and generate SVG charts.
//!
//! Usage:
//!   chart-gen [--repo-path <path>] [--output-dir <dir>]
//!
//! Parses README.md at each git commit to extract test suite pass counts,
//! then generates SVG line charts showing progress over time.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Test suite counts at a point in time.
#[derive(Debug, Clone, Default)]
struct Counts {
    demos: u32,
    demos_total: u32,
    subset: u32,
    subset_total: u32,
    chibicc: u32,
    chibicc_total: u32,
    beej: u32,
    beej_total: u32,
    bgc: u32,
    bgc_total: u32,
    regrs: u32,
}

/// A data point: date string → counts.
type History = BTreeMap<String, Counts>;

fn main() {
    let repo_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());
    let output_dir = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "docs".to_string());

    let history = extract_history(Path::new(&repo_path));
    if history.is_empty() {
        eprintln!("No data points found in git history");
        std::process::exit(1);
    }

    eprintln!("Extracted {} data points", history.len());

    let out = PathBuf::from(&output_dir);

    // Individual charts per suite
    generate_suite_chart(&history, &out.join("chart-chibicc.svg"), "chibicc", 41,
        |c| c.chibicc, "#2196F3");
    generate_suite_chart(&history, &out.join("chart-beej.svg"), "beej-c-guide", 11,
        |c| c.beej, "#4CAF50");
    generate_suite_chart(&history, &out.join("chart-bgc.svg"), "bgc", 117,
        |c| c.bgc, "#FF9800");
    generate_count_chart(&history, &out.join("chart-demos.svg"), "Demos",
        |c| c.demos, "#9C27B0");
    generate_count_chart(&history, &out.join("chart-regrs.svg"), "reg-rs Regression Tests",
        |c| c.regrs, "#607D8B");

    // Combined overview (pass counts only, no totals)
    generate_test_chart(&history, &out.join("chart-test-progress.svg"));

    eprintln!("Charts written to {output_dir}/");
}

fn extract_history(repo: &Path) -> History {
    let output = Command::new("git")
        .args(["log", "--format=%H %ai", "--", "README.md"])
        .current_dir(repo)
        .output()
        .expect("git log failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut history = History::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }
        let hash = parts[0];
        let date = &parts[1][..10]; // YYYY-MM-DD

        let readme = Command::new("git")
            .args(["show", &format!("{hash}:README.md")])
            .current_dir(repo)
            .output();

        let Ok(readme) = readme else { continue };
        let content = String::from_utf8_lossy(&readme.stdout);

        if let Some(counts) = parse_counts(&content) {
            // Keep the latest entry per date (most recent commit)
            history.insert(date.to_string(), counts);
        }
    }

    history
}

fn parse_counts(readme: &str) -> Option<Counts> {
    let mut c = Counts::default();
    let mut found = false;

    for line in readme.lines() {
        let stripped: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if stripped.starts_with("|tc24rdemos") || stripped.starts_with("|cc24demos") {
            let nums = extract_two_numbers(&stripped);
            c.demos = nums.0;
            c.demos_total = nums.1;
            found = true;
        } else if stripped.starts_with("|chibicc-subset") || stripped.starts_with("|chibiccsubset") {
            let nums = extract_two_numbers(&stripped);
            c.subset = nums.0;
            c.subset_total = nums.1;
        } else if stripped.starts_with("|chibicc") && !stripped.contains("subset") {
            let nums = extract_two_numbers(&stripped);
            c.chibicc = nums.0;
            c.chibicc_total = nums.1;
        } else if stripped.starts_with("|beej") {
            let nums = extract_two_numbers(&stripped);
            c.beej = nums.0;
            c.beej_total = nums.1;
        } else if stripped.starts_with("|reg-rs") || stripped.starts_with("|reg-rsreg") {
            let nums = extract_two_numbers(&stripped);
            c.regrs = nums.0;
        } else if stripped.starts_with("|bgc") {
            let nums = extract_two_numbers(&stripped);
            c.bgc = nums.0;
            c.bgc_total = nums.1;
        }
    }

    if found { Some(c) } else { None }
}

/// Extract the first two numbers from a pipe-delimited table row.
fn extract_two_numbers(s: &str) -> (u32, u32) {
    let parts: Vec<&str> = s.split('|').collect();
    let mut nums = Vec::new();
    for part in &parts {
        if let Ok(n) = part.parse::<u32>() {
            nums.push(n);
        }
    }
    match nums.len() {
        0 => (0, 0),
        1 => (nums[0], nums[0]),
        _ => (nums[0], nums[1]),
    }
}

// ——— SVG Generation ———

const CHART_W: f64 = 860.0;
const CHART_H: f64 = 400.0;
const MARGIN_L: f64 = 60.0;
const MARGIN_R: f64 = 80.0;
const MARGIN_T: f64 = 40.0;
const MARGIN_B: f64 = 80.0;
const PLOT_W: f64 = CHART_W - MARGIN_L - MARGIN_R;
const PLOT_H: f64 = CHART_H - MARGIN_T - MARGIN_B;

struct Series<'a> {
    name: &'a str,
    color: &'a str,
    values: Vec<(f64, f64)>, // (x_frac, y_value)
}

/// Per-suite chart: pass count vs total with a goal line
fn generate_suite_chart(
    history: &History,
    path: &Path,
    suite_name: &str,
    total: u32,
    extract: fn(&Counts) -> u32,
    color: &str,
) {
    let dates: Vec<&String> = history.keys().collect();
    let n = dates.len();
    if n == 0 {
        return;
    }

    let y_max = (total as f64 / 10.0).ceil() * 10.0;
    let pass_label: &str = Box::leak(format!("{suite_name} pass").into_boxed_str());
    let mut series = vec![Series {
        name: pass_label,
        color,
        values: Vec::new(),
    }];

    for (i, date) in dates.iter().enumerate() {
        let x = if n > 1 {
            i as f64 / (n - 1) as f64
        } else {
            0.5
        };
        series[0].values.push((x, extract(&history[*date]) as f64));
    }

    let annotations = vec![("total", total as f64, color)];
    let title = format!("{suite_name} ({} / {total})", extract(history.values().last().unwrap()));
    let svg = render_svg_with_annotations(
        &title,
        &series,
        y_max,
        &dates.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        &annotations,
    );
    write_file(path, &svg);
}

/// Simple count chart (no total line — count IS the total)
fn generate_count_chart(
    history: &History,
    path: &Path,
    label: &str,
    extract: fn(&Counts) -> u32,
    color: &str,
) {
    let dates: Vec<&String> = history.keys().collect();
    let n = dates.len();
    if n == 0 {
        return;
    }

    let y_max = history
        .values()
        .map(|c| extract(c))
        .max()
        .unwrap_or(1) as f64;
    let y_max = (y_max / 10.0).ceil() * 10.0;

    let count_label: &str = Box::leak(format!("{label}").into_boxed_str());
    let mut series = vec![Series {
        name: count_label,
        color,
        values: Vec::new(),
    }];

    for (i, date) in dates.iter().enumerate() {
        let x = if n > 1 {
            i as f64 / (n - 1) as f64
        } else {
            0.5
        };
        series[0].values.push((x, extract(&history[*date]) as f64));
    }

    let title = format!("{label} ({})", extract(history.values().last().unwrap()));
    let svg = render_svg_with_annotations(
        &title,
        &series,
        y_max,
        &dates.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        &[],
    );
    write_file(path, &svg);
}

/// Chart 1: Pass counts only for chibicc, beej, bgc (no totals — they're flat and dominate scale)
fn generate_test_chart(history: &History, path: &Path) {
    let dates: Vec<&String> = history.keys().collect();
    let n = dates.len();
    if n == 0 {
        return;
    }

    // Y-axis max: highest pass count across all suites
    let y_max = history
        .values()
        .map(|c| c.chibicc.max(c.beej).max(c.bgc))
        .max()
        .unwrap_or(1) as f64;
    let y_max = (y_max / 10.0).ceil() * 10.0;

    let mut series = vec![
        Series { name: "chibicc pass", color: "#2196F3", values: Vec::new() },
        Series { name: "beej pass", color: "#4CAF50", values: Vec::new() },
        Series { name: "bgc pass", color: "#FF9800", values: Vec::new() },
    ];

    for (i, date) in dates.iter().enumerate() {
        let x = if n > 1 { i as f64 / (n - 1) as f64 } else { 0.5 };
        let c = &history[*date];
        series[0].values.push((x, c.chibicc as f64));
        series[1].values.push((x, c.beej as f64));
        series[2].values.push((x, c.bgc as f64));
    }

    // Add horizontal reference lines for totals (annotated on right edge)
    let annotations = vec![
        ("chibicc: 41", 41.0, "#2196F3"),
        ("beej: 11", 11.0, "#4CAF50"),
    ];

    let svg = render_svg_with_annotations(
        "Test Suite Pass Counts",
        &series,
        y_max,
        &dates.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        &annotations,
    );
    write_file(path, &svg);
}

/// Chart 2: Demos and subset tests — growth over time
fn generate_demo_chart(history: &History, path: &Path) {
    let dates: Vec<&String> = history.keys().collect();
    let n = dates.len();
    if n == 0 {
        return;
    }

    let y_max = history
        .values()
        .map(|c| c.demos.max(c.subset))
        .max()
        .unwrap_or(1) as f64;
    let y_max = (y_max / 10.0).ceil() * 10.0;

    let mut series = vec![
        Series { name: "demos", color: "#9C27B0", values: Vec::new() },
        Series { name: "subset pass", color: "#00BCD4", values: Vec::new() },
    ];

    for (i, date) in dates.iter().enumerate() {
        let x = if n > 1 { i as f64 / (n - 1) as f64 } else { 0.5 };
        let c = &history[*date];
        series[0].values.push((x, c.demos as f64));
        series[1].values.push((x, c.subset as f64));
    }

    let svg = render_svg_with_annotations(
        "Demos & Subset Tests",
        &series,
        y_max,
        &dates.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        &[],
    );
    write_file(path, &svg);
}

fn render_svg_with_annotations(
    title: &str,
    series: &[Series],
    y_max: f64,
    dates: &[&str],
    annotations: &[(&str, f64, &str)],
) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {CHART_W} {CHART_H}" font-family="sans-serif" font-size="12">"#
    ));
    svg.push('\n');

    // Background
    svg.push_str(&format!(
        r#"  <rect width="{CHART_W}" height="{CHART_H}" fill="white"/>"#
    ));
    svg.push('\n');

    // Title
    svg.push_str(&format!(
        r#"  <text x="{}" y="24" text-anchor="middle" font-size="16" font-weight="bold">{title}</text>"#,
        CHART_W / 2.0
    ));
    svg.push('\n');

    // Y-axis grid lines and labels
    let y_steps = 5;
    for i in 0..=y_steps {
        let val = y_max * i as f64 / y_steps as f64;
        let y = MARGIN_T + PLOT_H - (val / y_max * PLOT_H);
        svg.push_str(&format!(
            "  <line x1=\"{MARGIN_L}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"#eee\"/>",
            MARGIN_L + PLOT_W
        ));
        svg.push('\n');
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" text-anchor="end" font-size="10">{}</text>"#,
            MARGIN_L - 8.0,
            y + 4.0,
            val as u32
        ));
        svg.push('\n');
    }

    // X-axis date labels (show ~6 evenly spaced)
    let n = dates.len();
    let label_count = n.min(6);
    for i in 0..label_count {
        let idx = if label_count > 1 {
            i * (n - 1) / (label_count - 1)
        } else {
            0
        };
        let x = MARGIN_L + (idx as f64 / (n - 1).max(1) as f64) * PLOT_W;
        let label = if dates[idx].len() >= 10 {
            &dates[idx][5..10] // MM-DD
        } else {
            dates[idx]
        };
        svg.push_str(&format!(
            r#"  <text x="{x}" y="{}" text-anchor="middle" font-size="10" transform="rotate(-30,{x},{})">{label}</text>"#,
            MARGIN_T + PLOT_H + 20.0,
            MARGIN_T + PLOT_H + 20.0
        ));
        svg.push('\n');
    }

    // Plot area border
    svg.push_str(&format!(
        "  <rect x=\"{MARGIN_L}\" y=\"{MARGIN_T}\" width=\"{PLOT_W}\" height=\"{PLOT_H}\" fill=\"none\" stroke=\"#ccc\"/>"
    ));
    svg.push('\n');

    // Annotation lines (horizontal dashed lines for totals)
    for (label, val, color) in annotations {
        let y = MARGIN_T + PLOT_H - (val / y_max * PLOT_H);
        svg.push_str(&format!(
            "  <line x1=\"{MARGIN_L}\" y1=\"{y:.1}\" x2=\"{}\" y2=\"{y:.1}\" stroke=\"{color}\" stroke-width=\"1\" stroke-dasharray=\"4,4\" opacity=\"0.5\"/>",
            MARGIN_L + PLOT_W
        ));
        svg.push('\n');
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" font-size="9" fill="{color}" opacity="0.7">{label}</text>"#,
            MARGIN_L + PLOT_W + 4.0,
            y + 3.0
        ));
        svg.push('\n');
    }

    // Data lines
    for s in series {
        if s.values.is_empty() || y_max == 0.0 {
            continue;
        }
        let width = 2.5;

        let points: Vec<String> = s
            .values
            .iter()
            .map(|(xf, yv)| {
                let px = MARGIN_L + xf * PLOT_W;
                let py = MARGIN_T + PLOT_H - (yv / y_max * PLOT_H);
                format!("{px:.1},{py:.1}")
            })
            .collect();

        svg.push_str(&format!(
            r#"  <polyline points="{}" fill="none" stroke="{}" stroke-width="{width}"/>"#,
            points.join(" "),
            s.color
        ));
        svg.push('\n');

        // End-point dot and label
        if let Some((xf, yv)) = s.values.last() {
            let px = MARGIN_L + xf * PLOT_W;
            let py = MARGIN_T + PLOT_H - (yv / y_max * PLOT_H);
            svg.push_str(&format!(
                r#"  <circle cx="{px:.1}" cy="{py:.1}" r="3" fill="{}"/>"#,
                s.color
            ));
            svg.push('\n');
        }
    }

    // Legend
    let legend_x = MARGIN_L + 10.0;
    let legend_y = MARGIN_T + PLOT_H + 40.0;
    let cols = 3;
    let col_w = PLOT_W / cols as f64;
    for (i, s) in series.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let lx = legend_x + col as f64 * col_w;
        let ly = legend_y + row as f64 * 16.0;
        svg.push_str(&format!(
            r#"  <line x1="{lx}" y1="{ly}" x2="{}" y2="{ly}" stroke="{}" stroke-width="2"/>"#,
            lx + 20.0,
            s.color
        ));
        let last_val = s.values.last().map(|(_, v)| *v as u32).unwrap_or(0);
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" font-size="10">{} ({})</text>"#,
            lx + 24.0,
            ly + 4.0,
            s.name,
            last_val
        ));
        svg.push('\n');
    }

    svg.push_str("</svg>\n");
    svg
}

fn write_file(path: &Path, content: &str) {
    let mut f = std::fs::File::create(path).expect("cannot create file");
    f.write_all(content.as_bytes()).expect("write failed");
    eprintln!("Wrote {}", path.display());
}
