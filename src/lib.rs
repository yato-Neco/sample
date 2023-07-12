extern crate num;
mod command;
mod sound;
mod item;
use item::{item,Face};
use sound::{WORD,playsound};
use command::{command, delay, sleep_ms, COMMANDS};
#[macro_use]
extern crate num_derive;
use jni::{objects::JClass, sys::JNIEnv};
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom};



#[no_mangle]
pub unsafe extern "C" fn main() -> Vec<String> {
   use rand::Rng;
    use uuid::Uuid;
    COMMANDS.clear();

    let mut rng = rand::thread_rng();

    let id = Uuid::new_v4();
    let store = 0;


    command!("scoreboard objectives add Rcommand dummy");
    command!("scoreboard players set {} Rcommand {}",id.to_string(),store);

    for i in 0..12 {
        let r: i32 = rng.gen();
        command!("say random: {}", r);
        command!("scoreboard players set {} Rcommand {}",id.to_string(),i);
        delay!(100);
    }
    
    command!("scoreboard players reset {} Rcommand",id.to_string());
    return COMMANDS.clone();
}
