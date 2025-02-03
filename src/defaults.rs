pub fn show_issue_options(){
    println!();
    println!("Select task issue");
    println!("1) Epic");
    println!("2) Subtask");
}

pub fn show_status_options(is_config: bool){
    println!();
    if is_config{
        println!("Task Statues");
    }else{
        println!("Select task status");
    }
    println!("1) Draft");
    println!("2) Ready");
    println!("3) Todo");
    println!("4) In Progress");
    println!("5) Review");
    println!("6) Complete");
    println!("7) Archive");

}