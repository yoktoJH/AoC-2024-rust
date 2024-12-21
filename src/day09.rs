use std::fs;
use std::slice::ChunksExact;

pub(crate) fn day9(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read from file");
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    let mut files2:Vec<(usize,usize)> = Vec::new();
    let mut gaps2 :Vec<(usize,usize)>= Vec::new();
    let mut file_id = 0;
    for (i, c) in input.bytes().enumerate() {
        if c == b'\n' {
            continue;
        }

        if i & 1 == 1 {
            gaps.push(c - b'0');
            gaps2.push((file_id,(c-b'0') as usize))
        } else {
            files.push(c - b'0');
            files2.push((file_id,(c-b'0') as usize))
        }
        file_id += (c-b'0') as usize;
    }
    let mut checksum = 0;
    let mut file_id = 0;
    let mut files_front = 0;
    let mut files_back = files.len() - 1;
    let mut gaps_index = 0;
    while files_front <= files_back && files[files_front] != 0 {
        //first do the file part
        while files[files_front] != 0 {
            checksum += files_front * file_id;
            file_id += 1;
            files[files_front] -= 1;
        }
        files_front += 1;
        //then fill the gap
        while gaps[gaps_index] != 0 {
            if files_front >= files_back && files[files_front] == 0 {
                break;
            }
            checksum += files_back * file_id;
            file_id += 1;
            files[files_back] -= 1;
            gaps[gaps_index] -= 1;

            if files[files_back] == 0 {
                files_back -= 1;
            }
        }
        gaps_index += 1;
    }


    //part2
    let mut checksum2 =0;
    let mut files_back = files2.len()-1;
    while files_back > 0 {
        //find big enough gap
        let mut i =0;
        while i < files_back && gaps2[i].1 < files2[files_back].1{
            i+=1;
        }
        if i >= files_back {
            files_back-=1;
            continue;
        }
        //count the checksum
        let file_id = gaps2[i].0;
        let file_size = files2[files_back].1;
        let sum = (file_id+ file_id + file_size-1)*file_size/2;
            checksum2+= sum*files_back;

        //update the gap and the file tuple
        gaps2[i] = (gaps2[i].0+file_size, gaps2[i].1-file_size);
        files2[files_back] = (0,0);

        files_back-=1
    }
    // count all unmoved files
    for (i,file) in files2.iter().enumerate() {
        if *file == (0,0) {
            continue;
        }
        let (file_id,size) = file;
        let sum = (file_id + file_id+ size -1)*size/2;
        checksum2 += i * sum;
    }

    println!("Solution for day 9 part 1: {}", checksum);
    println!("Solution for day 9 part 2: {}", checksum2);
}
