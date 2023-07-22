extern crate num;
use rcommand::commands::*;
use rcommand::sound::*;
use rcommand::command;
use rcommand::delay;
use rcommand::playsound;
#[macro_use]
extern crate num_derive;
use jni::{objects::JClass, sys::JNIEnv};
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom};



#[no_mangle]
pub unsafe extern "C" fn mini(words:&Vec<&str>) -> Vec<String> {
    use rand::Rng;
    use uuid::Uuid;
    COMMANDS.clear();

    let mut rng = rand::thread_rng();

    let mut rand: [usize; 4] = [0,1,2,3];
    let mut word :[WORD;4] = [WORD::Sword;4];


    for i in 0..4 {
        let p = words[i + 1].parse().unwrap_or(0);
        word[i] = num::FromPrimitive::from_usize(p).unwrap();
    }

    let id = Uuid::new_v4().to_string();

    rand.shuffle(&mut rng);

    command!("setblock ~ ~2 ~ air");
    
    for p in 0..4 {
        command!(
            "scoreboard players set {} Rcommand {}",
            id.to_string(),
            rand[p]
        );


        playsound!(word[rand[p]],16);

        delay!(3500);
        for i in (0..5).rev() {
            

            if i < 2 {
                command!("fill ~-1 ~1 ~{} ~-12 ~4 ~{} minecraft:glass", i, i);
                command!(
                    "fill ~-1 ~1 ~{} ~-12 ~4 ~{} minecraft:glass",
                    i * -1,
                    i * -1
                );
            }else{
                command!("fill ~-1 ~1 ~{} ~-12 ~4 ~{} minecraft:white_concrete", i, i);
                command!(
                    "fill ~-1 ~1 ~{} ~-12 ~4 ~{} minecraft:white_concrete",
                    i * -1,
                    i * -1
                );
            }


            command!(
                "playsound minecraft:block.note_block.hat master @a[distance=0..12] ~ ~ ~ 2 1"
            );
            delay!(1000);

            if i == 0 {
                //不正解
                command!("execute unless score {} Rcommand matches 0 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:yellow_wool run playsound minecraft:tango.wrong master @s ~ ~ ~ 100 1",id);
                command!("execute unless score {} Rcommand matches 1 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:red_wool run playsound minecraft:tango.wrong master @s ~ ~ ~ 100 1",id);
                command!("execute unless score {} Rcommand matches 2 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:green_wool run playsound minecraft:tango.wrong master @s ~ ~ ~ 100 1",id);
                command!("execute unless score {} Rcommand matches 3 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:blue_wool run playsound minecraft:tango.wrong master @s ~ ~ ~ 100 1",id);
                
                //正解
                command!("execute if score {} Rcommand matches 0 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:yellow_wool run playsound minecraft:tango.answer master @s ~ ~ ~ 100 1",id);
                command!("execute if score {} Rcommand matches 1 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:red_wool run playsound minecraft:tango.answer master @s ~ ~ ~ 100 1",id);
                command!("execute if score {} Rcommand matches 2 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:green_wool run playsound minecraft:tango.answer master @s ~ ~ ~ 100 1",id);
                command!("execute if score {} Rcommand matches 3 run execute as @a[distance=0..16] at @s if block ~ ~-0.1 ~ minecraft:blue_wool run playsound minecraft:tango.answer master @s ~ ~ ~ 100 1",id);

                command!("fill ~-1 ~1 ~4 ~-12 ~4 ~-4 minecraft:air");
            }
        }

        
    }
    //command!("kill @e[type=!minecraft:player,distance=0..16]");
    command!("scoreboard players reset {} Rcommand",id.to_string());
    command!("setblock ~ ~2 ~ minecraft:stone_button[face=floor]");

    return COMMANDS.clone();
}
