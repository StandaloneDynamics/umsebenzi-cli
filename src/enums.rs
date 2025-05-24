use anyhow::{anyhow, Result};
use std::fmt;
use serde::{self, Deserialize, Serialize};

use colored::Colorize;

#[derive(PartialEq, Clone)]
pub enum Issue {
    EPIC,
    SUBTASK,
}

impl Issue {
    pub fn to_value(&self) -> i32 {
        match *self {
            Self::EPIC => 1,
            Self::SUBTASK => 2,
        }
    }
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "1" => Ok(Issue::EPIC),
            "2" => Ok(Issue::SUBTASK),
            _ => Err(anyhow!("Invalid issue")),
        }
    }
    // API returns string variant eg: "EPIC".
    // this is only used in the editing of tasks
    pub fn from_api_str(s: &str) -> Result<Self> {
        match s {
            "EPIC" => Ok(Issue::EPIC),
            "SUBTASK" => Ok(Issue::SUBTASK),
            _ => Err(anyhow!("Invalid Status")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status{
    DRAFT,
    READY,
    TO_DO,
    IN_PROGRESS,
    REVIEW,
    COMPLETE,
    ARCHIVE
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::DRAFT => write!(f, "{}", "DRAFT".blue()),
            Status::READY => write!(f, "{}", "READY".blue()),
            Status::TO_DO => write!(f, "{}", "TO_DO".yellow()),
            Status::IN_PROGRESS => write!(f, "{}", "IN_PROGRESS".green()),
            Status::REVIEW => write!(f, "{}", "REVIEW".magenta()),
            Status::COMPLETE => write!(f,"{}", "COMPLETE".magenta()),
            Status::ARCHIVE => write!(f, "{}", "ARCHIVE".magenta()),
        }
    }
}

impl Status {
    pub fn to_value(&self) -> i32 {
        match *self {
            Self::DRAFT => 1,
            Self::READY => 2,
            Self::TO_DO => 3,
            Self::IN_PROGRESS => 4,
            Self::REVIEW => 5,
            Self::COMPLETE => 6,
            Self::ARCHIVE => 7,
        }
    }
    // number repesenting status that user will enter
    pub fn from_str(s: &str) -> Result<Status> {
        match s {
            "1" => Ok(Status::DRAFT),
            "2" => Ok(Status::READY),
            "3" => Ok(Status::TO_DO),
            "4" => Ok(Status::IN_PROGRESS),
            "5" => Ok(Status::REVIEW),
            "6" => Ok(Status::COMPLETE),
            "7" => Ok(Status::ARCHIVE),
            _ => Err(anyhow!("Invalid task status")),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_to_value() {
        let epic = Issue::EPIC;
        assert_eq!(epic.to_value(), 1);
    }

    #[test]
    fn issue_from_str_wrong() {
        let epic = Issue::from_str("cats");
        assert!(epic.is_err());
    }

    #[test]
    fn issue_from_str_correct() {
        let epic = Issue::from_str("1");
        assert!(epic.is_ok());
    }

    #[test]
    fn issue_from_api_correct() {
        let epic = Issue::from_api_str("EPIC");
        assert!(epic.is_ok());
    }

    #[test]
    fn issue_from_api_wrong() {
        let epic = Issue::from_api_str("WRONG");
        assert!(epic.is_err());
    }
}
