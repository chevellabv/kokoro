use bincode::{config::standard, encode_to_vec};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Parse NPY file header
fn parse_npy_header(file: &mut File) -> anyhow::Result<(Vec<usize>, String)> {
    // Read magic
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;
    if &magic != b"\x93NUMPY" {
        anyhow::bail!("Not a valid NPY file");
    }

    // Read version
    let mut version = [0u8; 2];
    file.read_exact(&mut version)?;

    // Read header length
    let header_len = if version[0] == 1 {
        let mut buf = [0u8; 2];
        file.read_exact(&mut buf)?;
        u16::from_le_bytes(buf) as usize
    } else {
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;
        u32::from_le_bytes(buf) as usize
    };

    // Read header
    let mut header = vec![0u8; header_len];
    file.read_exact(&mut header)?;
    let header_str = String::from_utf8_lossy(&header);

    // Parse shape and dtype
    // Example: "{'descr': '<f4', 'fortran_order': False, 'shape': (510, 1, 256), }"
    let shape_start = header_str.find("'shape':").ok_or_else(|| anyhow::anyhow!("No shape in header"))?;
    let shape_part = &header_str[shape_start..];
    let shape_start_paren = shape_part.find('(').ok_or_else(|| anyhow::anyhow!("No shape tuple"))?;
    let shape_end_paren = shape_part.find(')').ok_or_else(|| anyhow::anyhow!("No shape tuple end"))?;
    let shape_str = &shape_part[shape_start_paren+1..shape_end_paren];

    let shape: Vec<usize> = shape_str
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    let dtype_start = header_str.find("'descr':").ok_or_else(|| anyhow::anyhow!("No descr in header"))?;
    let dtype_part = &header_str[dtype_start+8..];
    let dtype_start_quote = dtype_part.find('\'').ok_or_else(|| anyhow::anyhow!("No dtype"))?;
    let dtype_part = &dtype_part[dtype_start_quote+1..];
    let dtype_end_quote = dtype_part.find('\'').ok_or_else(|| anyhow::anyhow!("No dtype end"))?;
    let dtype = dtype_part[..dtype_end_quote].to_string();

    Ok((shape, dtype))
}

/// Load NPY file and return as 3D Vec
fn load_npy(path: &Path) -> anyhow::Result<Vec<Vec<Vec<f32>>>> {
    let mut file = File::open(path)?;
    let (shape, dtype) = parse_npy_header(&mut file)?;

    if dtype != "<f4" {
        anyhow::bail!("Expected dtype '<f4', got '{}'", dtype);
    }

    if shape.len() != 3 {
        anyhow::bail!("Expected 3D array, got {} dimensions", shape.len());
    }

    let total_elements = shape.iter().product::<usize>();
    let mut data = vec![0f32; total_elements];

    // Read all f32 values
    for val in &mut data {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)?;
        *val = f32::from_le_bytes(bytes);
    }

    // Reshape to 3D
    let mut result = Vec::with_capacity(shape[0]);
    let mut idx = 0;
    for _ in 0..shape[0] {
        let mut layer = Vec::with_capacity(shape[1]);
        for _ in 0..shape[1] {
            let mut row = Vec::with_capacity(shape[2]);
            for _ in 0..shape[2] {
                row.push(data[idx]);
                idx += 1;
            }
            layer.push(row);
        }
        result.push(layer);
    }

    Ok(result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../1.0".to_string());
    let output_file = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "../1.0/voices-rust.bin".to_string());

    println!("Loading NPY files from {}...", input_dir);

    let mut voices: HashMap<String, Vec<Vec<Vec<f32>>>> = HashMap::new();

    let dir = std::fs::read_dir(&input_dir)?;
    let mut npy_files: Vec<_> = dir
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("npy"))
        .collect();

    npy_files.sort_by_key(|e| e.file_name());

    for entry in npy_files {
        let path = entry.path();
        let voice_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
            .to_string();

        println!("Loading {}...", voice_name);
        let data = load_npy(&path)?;
        println!("  Shape: ({}, {}, {})",
            data.len(),
            data.get(0).map(|v| v.len()).unwrap_or(0),
            data.get(0).and_then(|v| v.get(0)).map(|v| v.len()).unwrap_or(0)
        );

        voices.insert(voice_name, data);
    }

    println!("\nEncoding {} voices to bincode...", voices.len());
    let encoded = encode_to_vec(&voices, standard())?;

    println!("Writing {} bytes to {}...", encoded.len(), output_file);
    std::fs::write(&output_file, &encoded)?;

    println!("Done! File size: {:.2} MB", encoded.len() as f64 / 1024.0 / 1024.0);

    Ok(())
}
