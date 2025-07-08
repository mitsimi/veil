use crate::png::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::Crc;
use std::fmt;
use std::io::{BufReader, Read};

/// Represents a PNG chunk, including its type, data, and CRC.
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Creates a new chunk with the given type and data, calculating the CRC.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;

        let mut chunk = Self {
            length,
            chunk_type,
            data,
            crc: 0, // Temporary value
        };

        chunk.crc = chunk.calculate_crc();
        chunk
    }

    /// Returns the length of the chunk data.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Returns a reference to the chunk type.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns a slice of the chunk data bytes.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the CRC value of the chunk.
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the chunk data as a UTF-8 string, or an error if invalid.
    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(|e| e.into())
    }

    /// Serializes the chunk to a vector of bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    /// Calculates the CRC for the chunk type and data.
    fn calculate_crc(&self) -> u32 {
        let crc_data: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_data)
    }
}

/// Implements conversion from a byte slice to a Chunk, validating the CRC.
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // Chunk is at least 12 bytes: 4 bytes for length, 4 bytes for chunk type, and 4 bytes for crc
        // chunk data can be 0 bytes
        if value.len() < 12 {
            return Err("Chunk is too short".into());
        }

        let mut reader = BufReader::new(value);
        let mut buffer: [u8; 4] = [0; 4];

        let _ = reader.read_exact(&mut buffer);
        let length: u32 = u32::from_be_bytes(buffer);

        let _ = reader.read_exact(&mut buffer);
        let chunk_type: ChunkType =
            ChunkType::try_from(buffer).map_err(|_| "Invalid chunk type")?;

        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        let _ = reader.read_exact(&mut buffer);
        let crc: u32 = u32::from_be_bytes(buffer);

        let chunk = Self {
            length,
            chunk_type,
            data,
            crc,
        };

        // Validate CRC
        let expected_crc = chunk.calculate_crc();
        if chunk.crc != expected_crc {
            return Err("Invalid CRC".into());
        }

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::png::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
