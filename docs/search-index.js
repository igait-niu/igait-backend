var searchIndex = new Map(JSON.parse('[\
["igait_backend",{"t":"CCHQQQQCCHHCCCCCFONNNNNNNNNNNNNNNNHHHHFFPPFFGIPPFPPFONNNNNNNNNNNNNNONNNNNNOONNNNNONONNNNNNNNNNNNNNNNNNNOONNNNNNNNONNNNNNOOOOONNNNNNNNNNNNNNNNNNNNNNNNOOOHCCCFNNHNONOOONNNOHFNNNHHNNNNOHFFONNNNOOONNOONNOHOONNNNNNOHHO","n":["daemons","helper","main","print_be","print_db","print_metis","print_s3","routes","filesystem","check_dir","work_queue","database","email","lib","metis","print","Database","_state","borrow","borrow_mut","count_jobs","ensure_user","fmt","from","get_all_jobs","get_job","get_status","init","into","new_job","try_from","try_into","type_id","update_status","send_email","send_failure_email","send_success_email","send_welcome_email","AppError","AppState","Complete","InferenceErr","Job","JobStatus","JobStatusCode","JobTaskID","Processing","Queue","Request","SubmissionErr","Submitting","User","age","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bucket","clone","clone","clone","clone_into","clone_into","clone_into","code","db","deserialize","deserialize","deserialize","deserialize","deserialize","email","eq","ethnicity","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from_ref","from_ref","from_ref","height","id","into","into","into","into","into","into","into","into_response","jobs","new","serialize","serialize","serialize","serialize","serialize","sex","status","status","task_number","timestamp","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","uid","value","weight","query_metis","completion","historical","upload","CompletionRequestArguments","borrow","borrow_mut","completion_entrypoint","from","igait_access_key","into","job_id","status_code","status_content","try_from","try_into","type_id","uid","unpack_completion_arguments","HistoricalRequestArguments","borrow","borrow_mut","from","get_email_and_pdf_link","historical_entrypoint","into","try_from","try_into","type_id","uid","unpack_historical_arguments","UploadRequestArguments","UploadRequestFile","age","borrow","borrow","borrow_mut","borrow_mut","bytes","email","ethnicity","from","from","front_file","height","into","into","name","save_upload_files","sex","side_file","try_from","try_from","try_into","try_into","type_id","type_id","uid","unpack_upload_arguments","upload_entrypoint","weight"],"q":[[0,"igait_backend"],[8,"igait_backend::daemons"],[9,"igait_backend::daemons::filesystem"],[11,"igait_backend::helper"],[16,"igait_backend::helper::database"],[34,"igait_backend::helper::email"],[38,"igait_backend::helper::lib"],[152,"igait_backend::helper::metis"],[153,"igait_backend::routes"],[156,"igait_backend::routes::completion"],[171,"igait_backend::routes::historical"],[183,"igait_backend::routes::upload"],[213,"anyhow"],[214,"tokio::sync::mutex"],[215,"alloc::sync"],[216,"tokio::fs::read_dir"],[217,"core::fmt"],[218,"alloc::vec"],[219,"core::result"],[220,"core::any"],[221,"chrono::offset::utc"],[222,"chrono::datetime"],[223,"serde::de"],[224,"core::convert"],[225,"axum_core::body"],[226,"axum_core::response"],[227,"serde::ser"],[228,"axum::extract::state"],[229,"axum::extract::multipart"],[230,"alloc::string"]],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,0,0,0,0,0,0,20,20,0,0,0,0,20,20,0,20,20,0,13,21,13,15,20,23,3,25,21,13,15,20,23,3,25,3,13,15,20,13,15,20,15,3,21,13,15,20,23,13,20,13,21,13,15,20,23,3,25,21,13,15,20,23,3,25,25,25,13,15,20,13,23,21,13,15,20,23,3,25,25,21,3,21,13,15,20,23,13,13,23,3,13,13,15,20,21,13,15,20,23,3,25,21,13,15,20,23,3,25,21,13,15,20,23,3,25,21,15,13,0,0,0,0,0,34,34,0,34,34,34,34,34,34,34,34,34,34,0,0,38,38,38,0,0,38,38,38,38,38,0,0,0,40,40,39,40,39,39,40,40,40,39,40,40,40,39,39,0,40,40,40,39,40,39,40,39,40,0,0,40],"f":"``{{}{{d{b}}}}``````{{{j{{h{f}}}}l}{{d{b}}}}{{{j{{h{f}}}}}b}```````{ce{}{}}0{{nA`Ab}{{d{Ad}}}}{{nA`Ab}{{d{b}}}}{{nAf}Ah}{cc{}}{{nA`Ab}{{d{{Al{Aj}}}}}}{{nA`AdAb}{{d{Aj}}}}{{nA`AdAb}{{d{An}}}}{{}{{d{n}}}}8{{nA`AjAb}{{d{b}}}}{c{{B`{e}}}{}{}}0{cBb{}}{{nA`AdAnAb}{{d{b}}}}{{A`A`A`Ab}{{d{b}}}}{{A`An{Bf{Bd}}A`AdAb}{{d{b}}}}{{A`An{Bf{Bd}}AjA`A`A`AdAb}{{d{b}}}}{{AjA`AdAb}{{d{b}}}}```````````````{ce{}{}}0000000000000`{AjAj}{AnAn}{BhBh}{{ce}b{}{}}00``{c{{B`{Bj}}}Bl}{c{{B`{Aj}}}Bl}{c{{B`{An}}}Bl}{c{{B`{Bh}}}Bl}{c{{B`{Bn}}}Bl}`{{BhBh}C`}`{{BjAf}Ah}{{AjAf}Ah}{{AnAf}Ah}{{BhAf}Ah}{{BnAf}Ah}{{fAf}Ah}{{CbAf}Ah}{cc{}}00000{cCb{{Cf{Cd}}}}{Chc{}}2222``{ce{}{}}000000{Cb{{Cl{Cj}}}}`{{}{{d{f}}}}{{Bjc}B`Cn}{{Ajc}B`Cn}{{Anc}B`Cn}{{Bhc}B`Cn}{{Bnc}B`Cn}`````777{c{{B`{e}}}{}{}}0000000000000{cBb{}}000000```{{A`AdAb}{{d{b}}}}````::{{{D`{{j{{h{f}}}}}}Db}{{d{A`Cb}}}}>`;```332`{{DbAb}{{d{Dd}}}}`<<?{{{j{{h{f}}}}{Al{Aj}}DfDhAb}{{d{{Dj{DfDf}}}}}}2=554`{{DbAb}{{d{Dl}}}}```>>>>```{cc{}}0``??`{{{j{{h{f}}}}DnDnA`AdAjAb}{{d{b}}}}``888877`{{DbAb}{{d{E`}}}}{{{D`{{j{{h{f}}}}}}Db}{{d{bCb}}}}`","D":"In","p":[[1,"unit"],[8,"Result",213],[5,"AppState",38],[5,"Mutex",214],[5,"Arc",215],[5,"DirEntry",216],[5,"Database",16],[1,"str"],[1,"u128"],[1,"usize"],[5,"Formatter",217],[8,"Result",217],[5,"Job",38],[5,"Vec",218],[5,"JobStatus",38],[6,"Result",219],[5,"TypeId",220],[5,"Utc",221],[5,"DateTime",222],[6,"JobStatusCode",38],[5,"User",38],[10,"Deserializer",223],[5,"Request",38],[1,"bool"],[5,"AppError",38],[5,"Error",213],[10,"Into",224],[1,"never"],[5,"Body",225],[8,"Response",226],[10,"Serializer",227],[5,"State",228],[5,"Multipart",229],[5,"CompletionRequestArguments",156],[5,"String",230],[1,"u64"],[1,"tuple"],[5,"HistoricalRequestArguments",171],[5,"UploadRequestFile",183],[5,"UploadRequestArguments",183]],"r":[],"b":[],"c":"OjAAAAAAAAA=","e":"OzAAAAEAAIYAFQASAAIAFwAAAB8AAgApAAEALwABADIAAQA1ACYAYgABAGUABABxAAEAdAAkAJ4AAQCiAAAApAAGAK0AAQCzAAMAugAHAMQAAQDIAAAAygAIANUAAAA="}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
