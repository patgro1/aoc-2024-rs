use std::ops::Index;

advent_of_code::solution!(9);

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    File(u32),
    Free,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SizedBlock {
    File((u32, u32)),
    Free(u32),
}

pub fn parse_input(input: &str) -> Vec<Block> {
    let mut file_system: Vec<Block> = vec![];
    for (index, size) in input.trim().chars().enumerate() {
        if index % 2 == 0 {
            for _ in 0..size.to_digit(10).expect("Should be a number") {
                file_system.push(Block::File(index as u32 / 2));
            }
        } else {
            for _ in 0..size.to_digit(10).expect("Should be a number") {
                file_system.push(Block::Free);
            }
        }
    }

    file_system
}

pub fn parse_input_sized_blocks(input: &str) -> Vec<SizedBlock> {
    let mut file_system: Vec<SizedBlock> = vec![];
    for (index, size) in input.trim().chars().enumerate() {
        let size = size.to_digit(10).expect("This should be a number");
        if index % 2 == 0 {
            let fileidx = index as u32 / 2;
            file_system.push(SizedBlock::File((fileidx, size)));
        } else {
            file_system.push(SizedBlock::Free(size));
        }
    }

    file_system
}

pub fn defrag_by_block(file_system: Vec<Block>) -> Vec<Block> {
    let mut defrag_file_system = file_system.clone();
    let mut scan_idx = defrag_file_system.len() - 1;
    let mut insert_idx = 0;

    while insert_idx < scan_idx {
        if defrag_file_system[scan_idx] == Block::Free {
            scan_idx -= 1;
            continue;
        }
        match defrag_file_system[insert_idx] {
            Block::File(_) => {}
            Block::Free => {
                defrag_file_system.swap(insert_idx, scan_idx);
                scan_idx -= 1;
            }
        }
        insert_idx += 1;
    }

    defrag_file_system
}

pub fn defrag_by_file(file_system: Vec<SizedBlock>) -> Vec<SizedBlock> {
    let mut defrag_file_system = file_system.clone();

    let mut scan_idx = defrag_file_system.len();

    while scan_idx > 0 {
        scan_idx -= 1;
        if let SizedBlock::File((fileidx, filesize)) = defrag_file_system[scan_idx] {
            let mut insert_idx = 0;
            while insert_idx < scan_idx {
                match defrag_file_system[insert_idx] {
                    SizedBlock::File(_) => {}
                    SizedBlock::Free(size) => {
                        if size >= filesize {
                            // Replace the file at scan index with a free block
                            defrag_file_system[scan_idx] = SizedBlock::Free(filesize);
                            // Mut the free block to remove the space (or remove it completely if
                            // all the free space was taken)
                            if size == filesize {
                                defrag_file_system.remove(insert_idx);
                            } else {
                                defrag_file_system[insert_idx] = SizedBlock::Free(size - filesize);
                            }
                            // Insert a new file block at current insert index
                            defrag_file_system
                                .insert(insert_idx, SizedBlock::File((fileidx, filesize)));
                            break;
                        }
                    }
                }
                insert_idx += 1
            }
        }
    }

    defrag_file_system
}

pub fn convert_to_block_list(file_system: Vec<SizedBlock>) -> Vec<Block> {
    let mut block_file_system: Vec<Block> = vec![];
    for blocks in file_system.iter() {
        match blocks {
            SizedBlock::File((fileidx, filesize)) => {
                for _ in 0..*filesize {
                    block_file_system.push(Block::File(*fileidx))
                }
            }
            SizedBlock::Free(size) => {
                for _ in 0..*size {
                    block_file_system.push(Block::Free)
                }
            }
        }
    }

    block_file_system
}

pub fn compute_checksum(file_system: Vec<Block>) -> u64 {
    let mut checksum: u64 = 0;
    for (idx, block) in file_system.iter().enumerate() {
        if let Block::File(file_idx) = block {
            checksum += *file_idx as u64 * idx as u64;
        }
    }
    checksum
}

pub fn compute_checksum_block_file_system(file_system: Vec<SizedBlock>) -> u64 {
    let mut checksum: u64 = 0;
    let mut block_base_idx: u64 = 0;
    for block in file_system.iter() {
        match block {
            SizedBlock::File((fileidx, size)) => {
                for i in block_base_idx..block_base_idx + *size as u64 {
                    checksum += i * *fileidx as u64;
                }
                block_base_idx += *size as u64;
            }
            SizedBlock::Free(size) => {
                block_base_idx += *size as u64;
            }
        }
    }
    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let file_system = parse_input(input);
    let new_file_system = defrag_by_block(file_system);
    Some(compute_checksum(new_file_system))
}

pub fn part_two(input: &str) -> Option<u64> {
    let file_system = parse_input_sized_blocks(input);
    let new_file_system = defrag_by_file(file_system);
    Some(compute_checksum_block_file_system(new_file_system))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
