use anyhow::{bail, Result};
use miniz_oxide::deflate::compress_to_vec_zlib;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::{env, fs, io::Cursor, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMIT_DATE: &str = env!("VERGEN_GIT_COMMIT_DATE");
const COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");

fn main() {
    println!(
        "PalWorldSaveRepair v{} {} {}",
        VERSION, COMMIT_DATE, COMMIT_HASH
    );
    println!("https://github.com/YDKK/PalWorldSaveRepair");
    println!();

    let env_key = "PLAYERS_SAVE_PATH";
    let Ok(path) = &env::var(env_key) else {
        eprintln!("{} environment variable not found", env_key);
        process::exit(1);
    };

    let Ok(dir) = fs::read_dir(path) else {
        eprintln!("failed to read dir: {}", path);
        process::exit(1);
    };

    for file in dir
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(|x| x.path().into_os_string().into_string())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.ends_with(".sav"))
    {
        println!("processing {}", file);
        let Ok(data) = fs::read(&file) else {
            eprintln!("failed to read save file");
            continue;
        };

        let decompressed_save = match decompress(&data) {
            Err(error) => {
                eprintln!("failed to decompress save file: {}", error);
                continue;
            }
            Ok(result) => result,
        };

        let mut reader = Cursor::new(decompressed_save);
        let Ok(mut save) = uesave::Save::read(&mut reader) else {
            eprintln!("failed to read decompressed save file");
            continue;
        };
        const EXPECTED_SAVE_GAME_TYPE: &str = "/Script/Pal.PalWorldPlayerSaveGame";
        const CORRUPTED_SAVE_GAME_TYPE: &str = "None.PalWorldPlayerSaveGame";
        if save.root.save_game_type == EXPECTED_SAVE_GAME_TYPE {
            println!("save file is ok");
            continue;
        } else if save.root.save_game_type != CORRUPTED_SAVE_GAME_TYPE {
            eprintln!("save file is corrupted, but cannot be repaired with this tool");
            continue;
        } else {
            println!("save file is corrupted, repairing...");
            save.root.save_game_type = EXPECTED_SAVE_GAME_TYPE.to_string();
            let modified_save = Vec::<u8>::new();
            let mut writer = Cursor::new(modified_save);
            let Ok(_) = save.write(&mut writer) else {
                eprintln!("failed to write save");
                continue;
            };
            let compressed_save = match compress(writer.get_ref()) {
                Err(error) => {
                    eprintln!("failed to compress save file: {}", error);
                    continue;
                }
                Ok(result) => result,
            };

            let Ok(_) = fs::write(file, &compressed_save) else {
                eprintln!("failed to write save file");
                continue;
            };
            println!("save file repaired");
        }
    }
}

const MAGIC_BYTES: [u8; 3] = *b"PlZ";

fn decompress(data: &Vec<u8>) -> Result<Vec<u8>> {
    let decompressed_len = i32::from_le_bytes(data[0..4].try_into()?);
    let compressed_len = i32::from_le_bytes(data[4..8].try_into()?);
    let magic_bytes: [u8; 3] = data[8..11].try_into()?;
    let save_type = data[11];

    if magic_bytes != MAGIC_BYTES {
        bail!("invalid magic: {:?}", magic_bytes);
    }

    if ![0x31, 0x32].contains(&save_type) {
        bail!("invalid save type: {}", save_type);
    }

    let compressed_save_data = &data[12..];

    if save_type == 0x31 && compressed_save_data.len() != compressed_len as usize {
        bail!(
            "invalid compressed length: expected: {}, actual: {}",
            compressed_len,
            compressed_save_data.len()
        );
    }

    let mut decompressed_data = decompress_to_vec_zlib(compressed_save_data)?;

    if save_type == 0x32 {
        if decompressed_data.len() != compressed_len as usize {
            bail!(
                "invalid compressed length: expected: {}, actual: {}",
                compressed_len,
                decompressed_data.len(),
            );
        }

        let decompressed_data2 = decompress_to_vec_zlib(&decompressed_data)?;
        decompressed_data = decompressed_data2;
    }

    if decompressed_data.len() != decompressed_len as usize {
        bail!(
            "invalid decompressed length: expected: {}, actual: {}",
            decompressed_len,
            decompressed_data.len(),
        );
    }

    Ok(decompressed_data)
}

fn compress(data: &Vec<u8>) -> Result<Vec<u8>> {
    let decompressed_len = data.len();
    let compressed_data = compress_to_vec_zlib(&data, 6);
    let compressed_len = compressed_data.len();

    let mut save = Vec::with_capacity(12 + compressed_len);
    unsafe {
        save.set_len(12 + compressed_len);
    }
    save[0..4].copy_from_slice(&i32::to_le_bytes(decompressed_len as i32));
    save[4..8].copy_from_slice(&i32::to_le_bytes(compressed_len as i32));
    save[8..11].copy_from_slice(&MAGIC_BYTES);
    save[11] = 0x31;
    save[12..].copy_from_slice(&compressed_data);

    Ok(save)
}
