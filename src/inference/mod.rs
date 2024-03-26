use crate::database::Job;
use crate::print_be;
use tokio::fs::{
    remove_dir_all,
    read_to_string
};

pub async fn run_inference(folder_name: String) -> Result<(), String> {
    print_be(&format!("STARTING ON {folder_name}"));

    let job_data: String = read_to_string(&format!("data/queue/{folder_name}/data.json"))
        .await
        .expect("???");
    
    let job: Job = serde_json::from_str(&job_data)
        .map_err(|_| String::from("Failed to deserialize data! It's possible it was not well enough cleansed? Regardless, aborting job!"))?;
        
    println!("{job:?}");
    // some stuff
    
    // Finally, purge the directory
    if remove_dir_all(format!("data/queue/{}", folder_name)).await.is_err() {
        println!("FAILED TO REMOVE 'data/queue/{}'!", folder_name);
    };

    Ok(())
}