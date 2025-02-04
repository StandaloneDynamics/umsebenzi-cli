use anyhow::{Result, anyhow};

#[derive(PartialEq, Clone)]
pub enum Issue{
    EPIC,
    SUBTASK,
}

impl Issue {
    pub fn to_value(&self) -> i32{
        match *self{
            Self::EPIC => 1,
            Self::SUBTASK => 2
        }
    }
    pub fn from_str(s: &str) -> Result<Self>{
        match s{
            "1" => Ok(Issue::EPIC),
            "2" => Ok(Issue::SUBTASK),
            _ => Err(anyhow!("Invalid issue"))
        }
    }
    // API returns string variant eg: "EPIC".
    // this is only used in the editing of tasks
    pub fn from_api_str(s: &str) -> Result<Self>{
        match s{
            "EPIC" => Ok(Issue::EPIC),
            "SUBTASK" => Ok(Issue::SUBTASK),
            _ => Err(anyhow!("Invalid Status"))
        }
    }
    
}


pub enum TaskStatus{
    DRAFT,
    READY,
    TODO,
    InPROGRESS,
    REVIEW,
    COMPLETE,
    ARCHIVE
}

impl TaskStatus{
    pub fn to_value(&self) -> i32{
        match *self{
            Self::DRAFT => 1,
            Self::READY => 2,
            Self::TODO => 3,
            Self::InPROGRESS => 4,
            Self::REVIEW => 5,
            Self::COMPLETE => 6,
            Self::ARCHIVE => 7,   
        }

    }
    pub fn from_str(s: &str) -> Result<TaskStatus>{
        match s{
            "1" => Ok(TaskStatus::DRAFT),
            "2" => Ok(TaskStatus::READY),
            "3" => Ok(TaskStatus::TODO),
            "4" => Ok(TaskStatus::InPROGRESS),
            "5" => Ok(TaskStatus::REVIEW),
            "6" => Ok(TaskStatus::COMPLETE),
            "7" => Ok(TaskStatus::ARCHIVE),
            _ => Err(anyhow!("Invalid task status"))
        }

    }
    pub fn from_api_string(s: &str) -> Result<TaskStatus>{
        match s{
            "DRAFT" => Ok(TaskStatus::DRAFT),
            "READY" => Ok(TaskStatus::READY),
            "TODO" => Ok(TaskStatus::TODO),
            "IN_PROGRESS" => Ok(TaskStatus::InPROGRESS),
            "REVIEW" => Ok(TaskStatus::REVIEW),
            "COMPLETE" => Ok(TaskStatus::COMPLETE),
            "ARCHIVE" => Ok(TaskStatus::ARCHIVE),
            _ => Err(anyhow!("Invalid task status"))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_to_value(){
        let epic = Issue::EPIC;
        assert_eq!(epic.to_value(), 1);
    }

    #[test]
    fn issue_from_str_wrong(){
        let epic = Issue::from_str("cats");
        assert!(epic.is_err());
    }

    #[test]
    fn issue_from_str_correct(){
        let epic = Issue::from_str("1");
        assert!(epic.is_ok());
    }

    #[test]
    fn issue_from_api_correct(){
        let epic = Issue::from_api_str("EPIC");
        assert!(epic.is_ok());
    }

    #[test]
    fn issue_from_api_wrong(){
        let epic = Issue::from_api_str("WRONG");
        assert!(epic.is_err());
    }
}