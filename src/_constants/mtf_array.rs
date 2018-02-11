#![allow(dead_code)]

use std;
use std::io::Write;

pub fn generate() {
    let fvalue_dest_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_VALUE_ARRAY.txt");
    let findex_dest_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_INDEX_ARRAY.txt");
    let fnext_dest_path  = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_NEXT_ARRAY.txt");
    let mut fvalue = std::io::BufWriter::new(std::fs::File::create(&fvalue_dest_path).unwrap());
    let mut findex = std::io::BufWriter::new(std::fs::File::create(&findex_dest_path).unwrap());
    let mut fnext  = std::io::BufWriter::new(std::fs::File::create(&fnext_dest_path).unwrap());

    write!(fvalue, "[{}]", [
           32,  101, 116, 97,  105, 111, 110, 114, 115, 108, 104, 100, 99,  117, 93,  91,
           109, 112, 103, 102, 10,  121, 98,  39,  119, 46,  44,  118, 59,  38,  124, 47,
           49,  107, 61,  48,  67,  65,  58,  45,  84,  83,  60,  62,  50,  113, 73,  57,
           42,  120, 41,  40,  66,  77,  80,  69,  68,  53,  51,  72,  70,  56,  52,  71,
           82,  54,  76,  55,  78,  87,  122, 125, 123, 79,  106, 85,  74,  75,  208, 95,
           195, 35,  86,  215, 90,  34,  89,  209, 128, 224, 184, 131, 92,  227, 37,  33,
           176, 169, 206, 226, 130, 63,  88,  81,  161, 153, 43,  129, 188, 179, 216, 164,
           181, 189, 148, 190, 173, 187, 186, 229, 225, 167, 217, 177, 178, 168, 149, 185,
           197, 144, 147, 196, 207, 194, 180, 156, 132, 170, 166, 136, 182, 191, 9,   230,
           141, 160, 175, 36,  152, 140, 165, 145, 94,  133, 163, 183, 171, 157, 137, 174,
           134, 135, 236, 151, 231, 155, 201, 158, 138, 143, 150, 162, 159, 139, 172, 154,
           126, 232, 235, 146, 233, 228, 202, 203, 142, 214, 237, 204, 219, 234, 213, 96,
           218, 199, 64,  210, 239, 198, 211, 205, 212, 240, 222, 220, 200, 0,   1,   2,
           3,   4,   5,   6,   7,   8,   11,  12,  13,  14,  15,  16,  17,  18,  19,  20,
           21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,  127, 192, 193, 221, 223,
           238, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ].iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")).unwrap();

    write!(findex, "[{}]", [
           205, 206, 207, 208, 209, 210, 211, 212, 213, 142, 20,  214, 215, 216, 217, 218,
           219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234,
           0,   95,  85,  81,  147, 94,  29,  23,  51,  50,  48,  106, 26,  39,  25,  31,
           35,  32,  44,  58,  62,  57,  65,  67,  61,  47,  38,  28,  42,  34,  43,  101,
           194, 37,  52,  36,  56,  55,  60,  63,  59,  46,  76,  77,  66,  53,  68,  73,
           54,  103, 64,  41,  40,  75,  82,  69,  102, 86,  84,  15,  92,  14,  152, 79,
           191, 3,   22,  12,  11,  1,   19,  18,  10,  4,   74,  33,  9,   16,  6,   5,
           17,  45,  7,   8,   2,   13,  27,  24,  49,  21,  70,  72,  30,  71,  176, 235,
           88,  107, 100, 91,  136, 153, 160, 161, 139, 158, 168, 173, 149, 144, 184, 169,
           129, 151, 179, 130, 114, 126, 170, 163, 148, 105, 175, 165, 135, 157, 167, 172,
           145, 104, 171, 154, 111, 150, 138, 121, 125, 97,  137, 156, 174, 116, 159, 146,
           96,  123, 124, 109, 134, 112, 140, 155, 90,  127, 118, 117, 108, 113, 115, 141,
           236, 237, 133, 80,  131, 128, 197, 193, 204, 166, 182, 183, 187, 199, 98,  132,
           78,  87,  195, 198, 200, 190, 185, 83,  110, 122, 192, 188, 203, 238, 202, 239,
           89,  120, 99,  93,  181, 119, 143, 164, 177, 180, 189, 178, 162, 186, 240, 196,
           201, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ].iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")).unwrap();

    write!(fnext, "[{}]", [
           0,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11,  12,  13,  14,
           15,  16,  17,  18,  19,  20,  21,  21,  22,  23,  24,  25,  26,  27,  28,  29,
           30,  30,  31,  32,  33,  34,  35,  36,  36,  37,  38,  39,  40,  41,  41,  42,
           43,  44,  45,  46,  46,  47,  48,  49,  50,  50,  51,  52,  53,  53,  54,  55,
           56,  57,  57,  58,  59,  60,  60,  61,  62,  63,  63,  64,  65,  65,  66,  67,
           68,  68,  69,  70,  70,  71,  72,  73,  73,  74,  75,  75,  76,  77,  77,  78,
           79,  79,  80,  81,  81,  82,  83,  83,  84,  85,  85,  86,  87,  87,  88,  88,
           89,  90,  90,  91,  91,  92,  93,  93,  94,  94,  95,  96,  96,  97,  97,  98,
           99,  99,  100, 100, 101, 101, 102, 103, 103, 104, 104, 105, 105, 106, 106, 107,
           107, 108, 108, 109, 110, 110, 111, 111, 112, 112, 113, 113, 114, 114, 115, 115,
           116, 116, 117, 117, 118, 118, 119, 119, 120, 120, 120, 121, 121, 122, 122, 123,
           123, 124, 124, 125, 125, 125, 126, 126, 127, 127, 128, 128, 129, 129, 129, 130,
           130, 131, 131, 131, 132, 132, 133, 133, 134, 134, 134, 135, 135, 135, 136, 136,
           137, 137, 137, 138, 138, 139, 139, 139, 140, 140, 140, 141, 141, 141, 142, 142,
           143, 143, 143, 144, 144, 144, 145, 145, 145, 146, 146, 146, 147, 147, 147, 148,
           148, 148, 149, 149, 149, 150, 150, 150, 150, 151, 151, 151, 152, 152, 152, 153,
    ].iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")).unwrap();
}
