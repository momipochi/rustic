use std::fs::File;
use std::io::{self, BufReader, Read};

//riff chunk
pub const CHUNK_ID: &'static str = "RIFF";
pub const CHUNK_SIZE: &'static str = "----";
pub const FORMAT: &'static str = "WAVE";

//fmt sub-chunk
pub const SUBCHUNK1_ID: &'static str = "fmt ";
pub const SUBCHUNK1_SIZE: u32 = 16;
pub const AUDIO_FORMAT: u32 = 1;
pub const NUM_CHANNELS: u32 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const BYTE_RATE: u32 = SAMPLE_RATE * NUM_CHANNELS * (SUBCHUNK1_SIZE / 2);
pub const BLOCK_ALIGN: u32 = NUM_CHANNELS * (SUBCHUNK1_SIZE / 8);
pub const BIT_PER_SAMPLE: u32 = 16;

//data sub-chunk
pub const SUBCHUNK2_ID: &'static str = "data";
pub const SUBCHUNK2_SIZE: &'static str = "----";

const PATH: &str = "C:/Storage/YTmusic/ggst/The Town Inside Me [uQTBzmBDSv0].wav";
fn main() {
    println!("Hello, world!");
    let file = File::open(PATH).expect("Should have opened file");
    let mut bufreader = BufReader::new(file);
    let mut buffer = [0_u8;4];
    while let Ok(_bf) = bufreader.read(&mut buffer) {
        let s = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid utf-8 sequence: {}",e),
        };
        println!("{}", s);
    }
}

fn read_wave_header(bufreader: &mut BufReader<File>) -> Result<u64, &'static str> {
    let b_riff = read_4_bytes(bufreader).expect("Could not get 4 byte");
    if b"RIFF" != &b_riff[..] {
        return Err("Could not read wave header");
    }
    let file_len = read_le_u32(bufreader).expect("Could not get file_len");

    let b_wave = read_4_bytes(bufreader).expect("Could not get bWave");
    if b"WAVE" != &b_wave[..] {
        return Err("Could not read wave header");
    }
    Ok(file_len as u64 + 8)
}

fn read_4_bytes(buff: &mut BufReader<File>) -> io::Result<[u8; 4]> {
    let mut buf = [0_u8; 4];
    read_into(buff, &mut buf).expect("Could not read 4 bytes");
    Ok(buf)
}

fn read_into(bf: &mut BufReader<File>, buf: &mut [u8]) -> io::Result<()> {
    let mut n = 0;
    while n < buf.len() {
        let progress = bf.read(buf).expect("Could not read buffer");
        if progress > 0 {
            n += progress;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to read enough bytes.",
            ));
        }
    }
    Ok(())
}

fn read_le_u32(bf: &mut BufReader<File>) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    read_into(bf, &mut buf).expect("Could not read le u32");
    Ok((buf[3] as u32) << 24 | (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | (buf[0] as u32) << 0)
}
