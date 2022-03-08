use s3::creds::Credentials;
use s3::{Bucket, S3Error};
use std::fs::File;
use std::io::prelude::*;

use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), S3Error> {
    println!("Hello, world!");
    let b = Bucket::new(
        "oblivious-narek",
        s3::Region::Custom{ region: "custom".to_string(), endpoint: "http://s3.amazonaws.com".to_string() },
        Credentials::from_profile(None)?,
    )?;


    let mut buf = String::new();
    let mut futs = FuturesUnordered::new();

    let mut s3obj_names = File::open("file_names.txt")?;
    s3obj_names.read_to_string(&mut buf)?;

    let names: Vec<&str>  = buf.split('\n').collect();
    let names = &names[..100];
    for n in names {
        let l = b.get_object(format!("winter/{}", n));
        futs.push(l);
    }

    println!{"sent {} futures", futs.len()};

    loop {
        println!("trying...");

        match futs.next().await  {
            Some(_handled) => {
                let (file,s) = _handled?;
                println!("file cont {:?} status {}", &file[0..10], s);
            },
            None => {
                println!("Got none");
                break;
            },
        }
    }

    Ok(())
}
