extern crate alphanumeric_sort;
extern crate num_cpus;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::{PathBuf, Path};
use asciify::AsciiBuilder;
use crate::util::command::Cmd;
use std::thread;
use itertools::Itertools;


pub struct AsciiVid{
    width: u16,
    height: u16,
}

impl AsciiVid{
    pub fn new(width:u16, height:u16)->Self{
        AsciiVid{
            width,
            height
        }
    }
    pub fn play(&self, input: String)-> Result<(), io::Error>{

        let input_folder = Path::new(&input);
        println!("Is Dir: {}", input_folder.is_dir());
        if input_folder.is_dir() {
            let mut sorted_dirs: Vec<PathBuf> = input_folder.read_dir()
                .unwrap()
                .map(|f| f.unwrap())
                .map(|f| f.path())
                .collect();
            
            alphanumeric_sort::sort_path_slice(&mut sorted_dirs);

            for path in sorted_dirs.into_iter() {
                if path.is_file(){
                    let f = File::open(path)?;
                    let f = BufReader::new(f);

                    print!("{}[2J", 27 as char);
                    for line in f.lines(){
                        print!("{}",line.unwrap());
                    }
                    thread::sleep(std::time::Duration::from_nanos(27191489));
                }
            }
        }
        Ok(())
    }

    pub fn render(self, input: String, output: String)-> std::io::Result<()>{
        let input_path = Path::new(&input);

        let mut sorted_dir: Vec<PathBuf> = input_path.read_dir()
            .unwrap()
            .map(|f| f.unwrap())
            .map(|f| f.path())
            .collect();
        
        alphanumeric_sort::sort_path_slice(&mut sorted_dir);
        
        if input_path.is_dir() {
            let mut handlers: Vec<_> = Vec::new();
            let chunk_size = sorted_dir.clone().len()/num_cpus::get();
            let path_chunks: Vec<Vec<PathBuf>> = sorted_dir.into_iter()
                .chunks(chunk_size)
                .into_iter()
                .map(|c| c.collect())
                .collect();
            let cloned_width = self.width;
            let cloned_height = self.height;
            for paths in path_chunks.clone() {
                let cloned_output = output.clone();
                handlers.push(thread::spawn(move||{
                    for path in paths.into_iter(){
                        if path.clone().is_file(){
                            let ascii_img = AsciiBuilder::new_from_path(path.clone())
                                .set_deep(false)
                                .set_invert(false)
                                .set_resize((cloned_width as u32, cloned_height as u32))
                                .build();
                            let filename = path.clone().file_name().unwrap().to_os_string().into_string().expect("Unable to open file");
                            let output_str = format!("{}/{}", cloned_output, filename);
                            println!("Rendering: {}", filename);
                            let mut output_path: PathBuf = PathBuf::from(output_str);
                            output_path.set_extension("txt");
                            let mut output_folder: File = File::create(output_path).expect("");
                            output_folder.write_all(&ascii_img.as_bytes()).expect("Unable to write in the file");
                        }
                    }
                }));
            }
            for h in handlers.into_iter(){
                h.join().unwrap();
            }
        }
        Ok(())
    }

    pub fn resize(&self, input: String, ouput: String){
        let output_format = format!("scale={}:{}", self.width, self.height);
        let args = vec![
            "-i",
            &input,
            "-vf",
            &output_format,
            "-crf",
            "18",
            &ouput
        ];
        Cmd::execute("ffmpeg", args);
    }

    pub fn split(&self, input: String, output: String, format: String){
        let output_format = format!("{}/{}",output, format);
        let args = vec![
            "-i",
            &input,
            &output_format
        ];
        Cmd::execute("ffmpeg", args);
    }
}