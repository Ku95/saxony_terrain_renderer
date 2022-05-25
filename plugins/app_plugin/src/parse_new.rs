use image::{imageops, GenericImage, ImageBuffer, Luma};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const MAX_HEIGHT: f32 = 1000.0;
const DGM_01_SCALE: usize = 1;
const DGM_01_DIMENSION: usize = 2000 / DGM_01_SCALE;
const DGM_20_SCALE: usize = 20;
const DGM_20_DIMENSION: usize = 2000 / DGM_20_SCALE;
const DGM_20_TILE_SIZE: u32 = 128 * 100;

fn parse_xyz(
    file_path: PathBuf,
    origin_x: usize,
    origin_y: usize,
    dimension: usize,
    scale: usize,
) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let mut data = vec![0; (dimension * dimension) as usize];

    let file = File::open(file_path).expect("Unable to open file.");
    let reader = BufReader::new(file);

    for line in reader.lines().map(Result::unwrap) {
        let mut coordinate = line.split_whitespace();

        let x = (coordinate
            .next()
            .and_then(|value| value.split_once('.'))
            .and_then(|(value, _)| value.parse::<usize>().ok())
            .unwrap()
            - origin_x)
            / scale;
        let y = (coordinate
            .next()
            .and_then(|value| value.split_once('.'))
            .and_then(|(value, _)| value.parse::<usize>().ok())
            .unwrap()
            - origin_y)
            / scale;
        let height = (coordinate.next().unwrap().parse::<f32>().unwrap() / MAX_HEIGHT
            * u16::MAX as f32) as u16;

        data[y * dimension + x] = height;
    }

    ImageBuffer::from_vec(dimension as u32, dimension as u32, data).unwrap()
}

pub(crate) fn parse_dgm_01(input_directory: &str, output_directory: &str) {
    let paths = fs::read_dir(input_directory).expect("Could not find the input directory.");
    let count = paths.count();
    let paths = fs::read_dir(input_directory).expect("Could not find the input directory.");

    for (n, path) in paths.enumerate() {
        let dir_path = path.unwrap().path();

        let paths = fs::read_dir(dir_path).expect("Could not find the input directory.");
        for path in paths {
            let file_path = path.unwrap().path();

            if file_path.extension().unwrap() != "xyz" {
                continue;
            }

            let file_name = file_path
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let origin_x = &file_name[7..10].parse::<usize>().unwrap() * 1000;
            let origin_y = &file_name[11..15].parse::<usize>().unwrap() * 1000;

            let image = parse_xyz(
                file_path,
                origin_x,
                origin_y,
                DGM_01_DIMENSION,
                DGM_01_SCALE,
            );
            image
                .save(format!("{output_directory}/{file_name}.png"))
                .expect("Could not save file.");
        }

        println!("Finished parsing {n} of {count} files.");
    }
}

pub(crate) fn parse_dgm_20(input_directory: &str, output_directory: &str) {
    let paths = fs::read_dir(input_directory).expect("Could not find the input directory.");

    for path in paths {
        let file_path = path.unwrap().path();
        let file_name = file_path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let origin_x = &file_name[8..11].parse::<usize>().unwrap() * 1000;
        let origin_y = &file_name[12..16].parse::<usize>().unwrap() * 1000;

        let image = parse_xyz(
            file_path,
            origin_x,
            origin_y,
            DGM_20_DIMENSION,
            DGM_20_SCALE,
        );
        image
            .save(format!("{output_directory}/{file_name}.png"))
            .expect("Could not save file.");
    }
}

pub(crate) fn combine_dgm_20(input_directory: &str, output_file_path: &str) {
    let paths = fs::read_dir(input_directory).expect("Could not find the input directory.");

    let mut output = <ImageBuffer<Luma<u16>, _>>::new(DGM_20_TILE_SIZE, DGM_20_TILE_SIZE);

    for path in paths {
        let file_path = path.unwrap().path();
        let file_name = file_path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let x = (&file_name[8..11].parse::<usize>().unwrap() - 278) / 2;
        let y = (&file_name[12..16].parse::<usize>().unwrap() - 5560) / 2;

        let input = image::open(file_path).unwrap().into_luma16();
        // let input: &dyn GenericImageView<Pixel = Luma<u16>> = &input;

        output
            .copy_from(
                &input,
                (DGM_20_DIMENSION * x) as u32,
                (DGM_20_DIMENSION * y) as u32,
            )
            .unwrap();
    }

    imageops::flip_horizontal_in_place(&mut output);

    output.save(output_file_path).expect("Could not save file.");
}
