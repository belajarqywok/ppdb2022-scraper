use csv;
use reqwest;

pub fn get_school_profile (
    start : i64, range : i64
) -> Result <(), Box<dyn std::error::Error>> {

    // create "smp" directory
    // example : /datasets/smp/
    println!("[ + ] Create a directory for 'smp' datas. ");
    std::process::Command::new("cmd")
        .args(&["/C", "cd datasets && mkdir smp"])
        .output()
        .expect("failed to execute process");



    // create "prestasi akademik" directory
    // example : /datasets/smp/prestasi_akademik
    println!("[ + ] Create a directory for 'prestasi_akademik' datas. ");
    std::process::Command::new("cmd")
        .args(&["/C", "cd datasets/smp && mkdir prestasi_akademik"])
        .output()
        .expect("failed to execute process");



    // create "prestasi non-akademik" directory
    // example : /datasets/smp/prestasi_non-akademik
    println!("[ + ] Create a directory for 'prestasi_non-akademik' datas. ");
    std::process::Command::new("cmd")
        .args(&["/C", "cd datasets/smp && mkdir prestasi_non-akademik"])
        .output()
        .expect("failed to execute process");


    
    // create "participants" directory
    // example : /datasets/smp/prestasi_akademik/participants
    // example : /datasets/smp/prestasi_non-akademik/participants
    println!("[ + ] Create a directory for 'participants' datas. ");
    std::process::Command::new("cmd")
        .args(&["/C", "cd datasets/smp/prestasi_akademik && mkdir participants"])
        .output()
        .expect("failed to execute process");


    std::process::Command::new("cmd")
        .args(&["/C", "cd datasets/smp/prestasi_non-akademik && mkdir participants"])
        .output()
        .expect("failed to execute process");


    let mut academic_profiles     : Vec<[String; 14]> = Vec::new();
    let mut non_academic_profiles : Vec<[String; 14]> = Vec::new();

    let profiles : [&mut Vec<[String; 14]>; 2] = [
        &mut academic_profiles, &mut non_academic_profiles
    ];


    println!("[ + ] Fetch data from endpoints. ");

    for index_get_school in 0..range {

        let index_school : i64 = start + index_get_school + 1;

        let mut academic_endpoint : String = "https://ppdb.jakarta.go.id/seleksi/prestasi/smp/1-".to_string();
        academic_endpoint.push_str(index_school.to_string().as_str());
        academic_endpoint.push_str("-0.json");

        let mut non_academic_endpoint : String = "https://ppdb.jakarta.go.id/seleksi/prestasinonakademik/smp/1-".to_string();
        non_academic_endpoint.push_str(index_school.to_string().as_str());
        non_academic_endpoint.push_str("-0.json");

        let endpoints : [String; 2] = [
            academic_endpoint, non_academic_endpoint
        ];

        for index_endpoints in 0..endpoints.len() as usize {

            match reqwest::blocking::get(endpoints[index_endpoints].as_str()).unwrap().text() {

                Ok(response) => {
    
                    let get_response : Result<
                        serde_json::Value,
                        serde_json::Error
                    > = serde_json::from_str(response.as_str());
    
                    if get_response.is_ok() {
    
                        let parse_response : serde_json::Value = get_response.unwrap();
                        
                        let datas : [String; 14] = [
                            parse_response["sekolah"]["sekolah_id"].as_str().unwrap().to_string(),
                            parse_response["sekolah"]["siap_id"].as_str().unwrap().to_string(),
                            parse_response["sekolah"]["nama"].as_str().unwrap().to_string(),
                            parse_response["sekolah"]["npsn"].as_str().unwrap().to_string(),
                            parse_response["sekolah"]["is_negeri"].as_bool().unwrap().to_string(),
                            parse_response["sekolah"]["is_sbi"].as_bool().unwrap().to_string(),
                            parse_response["sekolah"]["k_kota"].as_i64().unwrap().to_string(),
                            parse_response["sekolah"]["k_propinsi"].as_i64().unwrap().to_string(),
                            parse_response["sekolah"]["kota"].as_str().unwrap().to_string(),
                            parse_response["sekolah"]["propinsi"].as_str().unwrap().to_string(),
                            parse_response["kompetensi"].as_str().unwrap().to_string(),
                            parse_response["enable"].as_bool().unwrap().to_string(),
                            parse_response["jml_pagu"].as_str().unwrap().to_string(),
                            parse_response["jml_luar"].as_i64().unwrap().to_string()
                        ];
    
                        (profiles[index_endpoints]).push(datas);


                        // participant Data

                        let mut participant_datas  : Vec<[String; 3]> = Vec::new();

                        let participants : usize = parse_response["data"].as_array().unwrap().len();

                        for index_participants in 0..participants {

                            participant_datas.push([
                                parse_response["data"][index_participants][3].as_str().unwrap().to_string(),
                                parse_response["data"][index_participants][4].as_str().unwrap().to_string(),
                                parse_response["data"][index_participants][5].as_str().unwrap().to_string()
                            ]);

                        }



                        let participant_paths : [&str; 2] = [
                            "./datasets/smp/prestasi_akademik/participants/",
                            "./datasets/smp/prestasi_non-akademik/participants/"
                        ];

                        let mut participants_location : String = participant_paths[index_endpoints].to_string();
                        participants_location.push_str(parse_response["sekolah"]["nama"].as_str().unwrap());
                        participants_location.push_str(".csv");

                        let mut participants : csv::Writer<
                            std::fs::File
                        > = csv::Writer::from_path(
                            participants_location.as_str()
                        )?;

                        participants.write_record([
                            "registration_number".to_string(), "student_name".to_string(), "student_grade".to_string()
                        ])?;
                            
                        for index_participants in 0..participant_datas.len() {

                            participants.write_record(&[
                                
                                &participant_datas[index_participants][0],  // registration_number
                                &participant_datas[index_participants][1],  // student_name
                                &participant_datas[index_participants][2]   // student_grade

                            ])?;

                        }

                        participants.flush()?;
    
                    }
    
                },
    
                Err(error_message) => println!("error parsing header: {error_message:?}")
            }

        }

    }

    println!("[ + ] Convert data into csv file. ");

    if academic_profiles.len() == non_academic_profiles.len() {

        let mut academic_writer : csv::Writer<
            std::fs::File
        > = csv::Writer::from_path(
            "./datasets/smp/prestasi_akademik/profiles.csv"
        )?;


        let mut non_academic_writer : csv::Writer<
            std::fs::File
        > = csv::Writer::from_path(
            "./datasets/smp/prestasi_non-akademik/profiles.csv"
        )?;


        let record_names : [String; 14] = [
            "sekolah_id".to_string(), "siap_id".to_string(), "nama".to_string(),
            "npsn".to_string(),"is_negeri".to_string(), "is_sbi".to_string(),
            "k_kota".to_string(), "k_propinsi".to_string(), "kota".to_string(),
            "propinsi".to_string(), "kompetensi".to_string(), "enable".to_string(),
            "jml_pagu".to_string(), "jml_luar".to_string()
        ];

        academic_writer.write_record(&record_names)?;
        non_academic_writer.write_record(&record_names)?;

        for index_data_record in 0..academic_profiles.len() as usize {

            let mut academic_writer_record     : Vec<&String> = Vec::new();
            let mut non_academic_writer_record : Vec<&String> = Vec::new();


            for index_writer in 0..record_names.len() {

                academic_writer_record.push(
                    &academic_profiles[index_data_record][index_writer]
                );

                non_academic_writer_record.push(
                    &non_academic_profiles[index_data_record][index_writer]
                );

            }

            academic_writer.write_record(&academic_writer_record)?;
            non_academic_writer.write_record(&non_academic_writer_record)?;

        }

        academic_writer.flush()?;
        non_academic_writer.flush()?;

    }
    
    Ok(())

}