use crate::resource::Resource;

pub fn download(target_dir: &String, to_download: &Resource) {
    let client = reqwest::blocking::Client::new();
    let head = client.head(&to_download.download_url).send().unwrap();
    println!("{}", head.url());


    // let bytes = reqwest::blocking::get(&to_download.download_url).unwrap()
    //     .bytes().unwrap()
    //     .to_vec();


    // // download to memory
    // let bytes = reqwest::blocking::get(&remote_file.url).unwrap()
    //     .bytes().unwrap()
    //     .to_vec();
    //
    // // hash temp file data
    // let temp_file_sha256 = hasher::sha256(&bytes);
    //
    // //build local file name
    // let local_file_path = fs::canonicalize(&dir.as_path()).unwrap()
    //     .into_os_string().to_str().unwrap()
    //     .to_string();
    //
    // // write bytes to local file
    // temp_file.write_all(&bytes).unwrap();
}

pub fn delete(target_dir: &String, to_delete: &Resource) {}