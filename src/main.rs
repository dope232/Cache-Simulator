use std::io;

const CACHE_CAPACITY: usize = 4;
const BLOCK_SIZE: usize = 4;
const MM_CAPACITY: usize = 1024;

#[derive(Clone)]
struct CacheBlock {
    tag: usize,
    data: [u8; BLOCK_SIZE],
    valid: bool,
}

impl CacheBlock {
    fn new() -> Self {
        CacheBlock {
            tag: 0,
            data: [0; BLOCK_SIZE],
            valid: false,
        }
    }
}

fn main() {
    println!("Cache Simulator- Simulating Hits, Misses");
    println!("Enter 1 for Direct, 2 for Set associative, 3 for Associative");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let mapping_type = match choice.trim().as_ref() {
        "1" => "DIRECT",
        "2" => "SET ASSOCIATIVE",
        "3" => "ASSOCIATIVE",
        _ => panic!("Invalid mapping type"),
    };

    let mut cache: Vec<CacheBlock> = vec![CacheBlock::new(); CACHE_CAPACITY];

    println!("Enter the memory trace of 8 elements");
    let size = 8;
    let mut memory_trace = [0; 8];
    for i in 0..size {
        let mut arr_input = String::new();
        io::stdin().read_line(&mut arr_input).expect("msg");
        let arr_num: usize = arr_input.trim().parse().expect("msg");

        memory_trace[i] = arr_num;
    }
    println!("Memory Trace given {:?}", memory_trace);
    let mut hits = 0;
    let mut misses = 0;
    let mut evictions = 0;

    for address in memory_trace {
        let (tag, index) = get_tags_and_index(address, mapping_type);
        match find_in_cache(&cache, tag, index, mapping_type) {
            Some(_block) => {
                hits += 1;
                println!("Hit for {:?}", address);
            }
            None => {
                misses += 1;
                println!("Miss for {:?}", address);
                let evicted_block = replace_block(&mut cache, tag, index, mapping_type);
                if let Some(block) = evicted_block {
                    evictions += 1;
                }
            }
        }
    }

    println!("Hits: {}", hits);
    println!("Misses: {}", misses);
    println!("Evictions: {}", evictions);
}

fn get_tags_and_index(address: usize, mapping_type: &str) -> (usize, usize) {
    match mapping_type {
        "DIRECT" => {
            let index = address % CACHE_CAPACITY;
            let tag = address / CACHE_CAPACITY;
            (tag, index)
        }
        "SET ASSOCIATIVE" => {
            let num_sets = CACHE_CAPACITY / 2; // 2-way
            let index = address % num_sets;
            let tag = address / num_sets;
            (tag, index)
        }
        "ASSOCIATIVE" => {
            let tag = address;
            let index = 0;
            (tag, index)
        }
        _ => panic!("Invalid mapping type"),
    }
}

fn find_in_cache<'a>(
    cache: &'a [CacheBlock],
    tag: usize,
    index: usize,
    mapping_type: &str,
) -> Option<&'a CacheBlock> {
    match mapping_type {
        "DIRECT" => cache.get(index).filter(|block| block.valid && block.tag == tag),
        "SET ASSOCIATIVE" => {
            let set = index * 2;
            cache.iter().skip(set).take(2).find(|block| block.valid && block.tag == tag)
        }
        "ASSOCIATIVE" => cache.iter().find(|block| block.valid && block.tag == tag),
        _ => panic!("Invalid mapping type"),
    }
}

fn replace_block<'a>(
    cache: &'a mut [CacheBlock],
    tag: usize,
    index: usize,
    mapping_type: &'a str,
) -> Option<CacheBlock> {
    match mapping_type {
        "DIRECT" => {
            let block = &mut cache[index];
            let old_block = std::mem::replace(block, CacheBlock {
                tag,
                data: [0; BLOCK_SIZE],
                valid: true,
            });
            Some(old_block)
        }
        "SET ASSOCIATIVE" => {
            let set = index * 2;
            cache.iter_mut().skip(set).take(2).find(|block| !block.valid).map(|block| {
                let old_block = std::mem::replace(block, CacheBlock {
                    tag,
                    data: [0; BLOCK_SIZE],
                    valid: true,
                });
                old_block
            })
        }
        "ASSOCIATIVE" => {
            let block = &mut cache[index];
            let old_block = std::mem::replace(block, CacheBlock {
                tag,
                data: [0; BLOCK_SIZE],
                valid: true,
            });
            Some(old_block)
        }
        _ => panic!("Invalid mapping type"),
    }
}
