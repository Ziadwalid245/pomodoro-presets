use std::{io, thread, time::Duration};
fn main() {
    println!("How long would you like to work for?");
    println!("1. Work for 25mins and have 5mins break");
    println!("1. Work for 30mins and have 10mins break");
    println!("3. Work for 50mins and have 15mins break");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("could not parse arguments");
    let choice_number: i32 = choice.trim().parse().expect("could not convert no");
    if choice_number == 1 {
        preset_1();
    }
    if choice_number == 2 {
        preset_2();
    }
    if choice_number == 3 {
        preset_3();
    } else {
        println!("you must input a number");
    }
}
fn preset_1() {
    let work_time = 1500;
    let break_time = 300;

    for count in (0..work_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = count % 60;
        let minutes = count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
    for break_count in (0..break_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = break_count % 60;
        let minutes = break_count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
}
fn preset_2() {
    let work_time = 1800;
    let break_time = 600;

    for count in (0..work_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = count % 60;
        let minutes = count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
    for break_count in (0..break_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = break_count % 60;
        let minutes = break_count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
}
fn preset_3() {
    let work_time = 3000;
    let break_time = 900;

    for count in (0..work_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = count % 60;
        let minutes = count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
    for break_count in (0..break_time).rev() {
        thread::sleep(Duration::from_secs(1));
        let seconds = break_count % 60;
        let minutes = break_count / 60;
        println!("00:{}:{}", minutes, seconds);
    }
}
