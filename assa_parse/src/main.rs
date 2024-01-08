use assa_parse::assa_file::AssaFile;

fn main() {
    let input = "Princess Mononoke (1997)_Subtitles02.ENG.ass";
    // let input = "[Commie] Mirai Nikki - 01 [55466DF0]_Track03.ass";
    // let assa_file = match AssaFile::from_file(input) {
    //     Ok(file) => file,
    //     Err(_) => panic!("Could not open file"),
    // };

    let assa_file = AssaFile::from_file(input).unwrap();

    // let mut x = &mut ass_file;
    // let y = &ass_file.events.unwrap()[0];
    // let a = &ass_file.events.unwrap();
    // let mut z = &mut ass_file.events.unwrap()[0];

    // println!("{}", assa_file.styles[3]);
    println!("'{}'", assa_file.aegisub_extradata);

    assa_file.save_file_as("./test.ass");
}
