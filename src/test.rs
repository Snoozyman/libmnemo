use crate::types::{NemoFile, NemoPoint};

pub fn test_mnemo_data() -> &'static str {
    const DATA: &str = "
Section Name,TypeShot,Length(m),Depth IN(m),Depth OUT(m),Heading IN(dd),Heading OUT(dd),Pitch IN(dd),Pitch OUT(dd),Left(m),Right(m),Up(m),Down(m),Temperature(°C),Time,Marker
B30,STD,4.84,2.82,2.70,54.6,38.0,-1.8,1.3,0.0,0.0,0.0,0.0,22.8,00/01/11 - 04:03:42
B30,STD,3.92,2.70,2.69,32.2,36.3,-0.3,-3.6,0.0,0.0,0.0,0.0,22.9,00/01/11 - 04:05:40
B30,STD,7.75,2.76,4.76,34.3,36.0,-14.0,-13.5,0.0,0.0,0.0,0.0,23.0,00/01/11 - 04:07:13
B30,STD,2.62,4.77,4.73,29.9,30.9,0.0,-1.3,0.0,0.0,0.0,0.0,23.1,00/01/11 - 04:08:46
B30,STD,2.40,4.69,4.68,32.3,30.9,-1.0,-5.4,0.0,0.0,0.0,0.0,23.1,00/01/11 - 04:10:04
B30,STD,8.60,4.69,5.79,41.2,44.8,-3.5,-1.1,0.0,0.0,0.0,0.0,23.2,00/01/11 - 04:10:34
";
    DATA
}
pub fn debug_nemopoint(nm: NemoPoint) {
    println!("Name: {} ", nm.name);
    println!("TypeShot: {:?} ", nm.typeshot);
    println!("Length: {} ", nm.length);
    println!("Depth: {:?} ", nm.depth);
    println!("Heading: {:?} ", nm.heading);
    println!("Pitch: {:?} ", nm.pitch);
    println!("Dimensions: {:?} ", nm.dimensions);
    println!("Temperature: {} ", nm.temperature);
    println!("Time: {}", nm.time);
    println!("Parsed time: {:?}", nm.parsed_time);
}

pub fn debug_nemofile(nmf: NemoFile) {
    println!("File: {}", nmf.filename);
    for p in nmf.points {
        debug_nemopoint(p);
    }
}
