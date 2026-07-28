use std::fs::{File, ReadDir, metadata};
use std::fs;
use std::io::{self};
use std::env;
use rodio::Decoder;
use substring::Substring;
use std::collections::HashMap;
fn main() {
    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the sink_handle is dropped.
    let mut sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    sink_handle.log_on_drop(false);
    // Note that the playback stops when the player is dropped
    let player = rodio::Player::connect_new(&sink_handle.mixer());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    let mut cur_path = match env::home_dir(){
        Some(path)=> path.display().to_string(),    
        None=> "/".to_string(),     
    };
    loop {
        println!("cur_path is: {}",&cur_path);
        if cur_path=="" {cur_path="/".to_string();}
        let mut paths = fs::read_dir(&cur_path).unwrap();
        let paths_vector = paths_to_vector(&mut paths);
        println!("To queue a song, select a song from the following or a directory to move folders (\"0\" to move up a directory, \"q\" to quit, \"s\" to skip)");
        let path_choice = io::stdin().lines().nth(0).unwrap().unwrap();
        if path_choice=="q" { break; }
        if path_choice=="s" {player.skip_one(); continue;}
        let mut bytes_vec=path_choice.into_bytes();
        let choice=bytes_to_u16(&mut bytes_vec);
        if choice!=0 && choice<(paths_vector.len()+1)as u16{
            let choice_string=paths_vector.get((choice-1) as usize).unwrap().to_string();
            if !is_directory(&choice_string) {
                queue_song(&player, &choice_string);
                continue;
            }
        }
        cur_path=append_paths(&cur_path, &paths_vector, choice);
        
    }
}
fn bytes_to_u16(bytes_vec: &mut Vec<u8>) -> u16 {
    let mut choice=0 as u16;
    let mut i=bytes_vec.len() as u16;
    for byte in bytes_vec {
        i-=1;
        *byte-=48;
        choice+=*byte as u16*(10_u16.pow(i.try_into().unwrap())); 
    }
    choice
}
fn append_paths(cur_path: &str, new_paths: &Vec<String>, choice: u16) -> String {
    if choice==0 {
        let bytes = cur_path.as_bytes();
        let mut i=0;
        let mut furthest=0;
        for byte in bytes {
            if *byte==47{
                furthest=i;
            }
            i+=1;
        }
        cur_path.to_string().substring(0, furthest).to_string()
    }else if choice < (new_paths.len()+1) as u16{
        new_paths.get((choice-1) as usize).unwrap().to_string()
    }else {
        cur_path.to_string()
    }
}
fn paths_to_vector(paths: &mut ReadDir) ->  Vec<String> { 
    let mut text_paths = Vec::new();
    let mut i=1;
    for path in paths {
        let cleaned_path=path.unwrap().path().display().to_string();
        
        if !check_hidden(&cleaned_path) {
            text_paths.push(cleaned_path);
            println!("{}: {}",i,  text_paths.last().unwrap());
            i+=1;
        }
    }
    text_paths
}
fn check_hidden(path: &String) -> bool {
    let mut i=path.len()-1;
    let mut file_extension="".to_string();
    let mut valid_extensions=HashMap::new();
    valid_extensions.insert( "3pm.".to_string(), "true".to_string());
    valid_extensions.insert( "vaw.".to_string(), "true".to_string());
    valid_extensions.insert( "a4m.".to_string(), "true".to_string());
    loop {
        if i==0 {
            return false;
        }
        let curr_char = path.chars().nth(i).unwrap_or(' ');
        i-=1;
        file_extension.push(curr_char);
        if curr_char=='.' && !valid_extensions.contains_key(&file_extension) {
            return true;
        }else if curr_char=='.' || curr_char=='/'{
            return false;
        }
    }
}
fn is_directory(file: &String) -> bool{
    println!("{}", file);
    let md=metadata(&file);
    md.expect("not a file").is_dir()
}fn queue_song(player: &rodio::Player, song_path: &String){
    let file = File::open(song_path).unwrap();
    let decoder = Decoder::try_from(file).unwrap();
    player.append(decoder);
}
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
