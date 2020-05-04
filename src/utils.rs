pub fn chunks_up_to(max_value: u32, chunk_size: usize) -> Vec<Vec<u32>> {
    chunks((0..max_value).collect(), chunk_size)
}

pub fn chunks(values: Vec<u32>, chunk_size: usize) -> Vec<Vec<u32>> {
    let chunk_count = ((values.len() as f32) / (chunk_size as f32)).ceil() as usize;
    let mut chunks = Vec::with_capacity(chunk_count);

    for chunk_start in (0..values.len()).step_by(chunk_size) {
        let mut chunk = Vec::with_capacity(chunk_size);
        let chunk_end = values.len().min(chunk_start + chunk_size);
        for i in chunk_start..chunk_end {
            chunk.push(values[i]);
        }
        chunks.push(chunk);
    }

    chunks
}

#[cfg(test)]
mod chunks_up_to_tests {
    use super::*;

    #[test]
    fn test_chunks_up_to() {
        let expected = vec![vec![0, 1], vec![2, 3]];
        assert_eq!(expected, chunks_up_to(4, 2));
    }
}

#[cfg(test)]
mod chunks_tests {
    use super::*;

    #[test]
    fn test_chunks_evenly() {
        let values: Vec<u32> = (0..4).collect();
        let expected = vec![vec![0, 1], vec![2, 3]];
        assert_eq!(expected, chunks(values, 2));
    }

    #[test]
    fn test_chunks_oddly() {
        let values: Vec<u32> = (0..8).collect();
        let expected = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]];
        assert_eq!(expected, chunks(values, 3));
    }
}
