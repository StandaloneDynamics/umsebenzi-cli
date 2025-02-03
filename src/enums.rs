use anyhow::{Result, anyhow};

#[derive(PartialEq)]
pub enum Issue{
    EPIC,
    SubTask,
}

impl Issue {
    pub fn to_value(&self) -> i32{
        match *self{
            Self::EPIC => 1,
            Self::SubTask => 2
        }
    }
    pub fn from_str(s: &str) -> Result<Issue>{
        match s{
            "1" => Ok(Issue::EPIC),
            "2" => Ok(Issue::SubTask),
            _ => Err(anyhow!("Invalid issue"))
        }
    }
    
}


pub enum TaskStatus{
    Draft,
    Ready,
    TODO,
    InPROGRESS,
    REVIEW,
    COMPLETE,
    ARCHIVE
}

impl TaskStatus{
    pub fn to_value(&self) -> i32{
        match *self{
            Self::Draft => 1,
            Self::Ready => 2,
            Self::TODO => 3,
            Self::InPROGRESS => 4,
            Self::REVIEW => 5,
            Self::COMPLETE => 6,
            Self::ARCHIVE => 7,   
        }

    }
    pub fn from_str(s: &str) -> Result<TaskStatus>{
        match s{
            "1" => Ok(TaskStatus::Draft),
            "2" => Ok(TaskStatus::Ready),
            "3" => Ok(TaskStatus::TODO),
            "4" => Ok(TaskStatus::InPROGRESS),
            "5" => Ok(TaskStatus::REVIEW),
            "6" => Ok(TaskStatus::COMPLETE),
            "7" => Ok(TaskStatus::ARCHIVE),
            _ => Err(anyhow!("Invalid task status"))
        }

    }
}