use serde::ser::{Serialize, Serializer, SerializeStruct};


#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {

    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string)
        }
    }
}

//This works because we just want the string
impl Serialize for TaskStatus {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}

//this should be good for structs?
// impl Serialize for TaskStatus {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, 
//     S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("TaskStatus", 
//                                                 1)?;
//         s.serialize_field("status", &self.stringify())?;
//         s.end()
//     }
// }
//This would also need a serialized struct
// #[derive(Serialize)]
// struct TaskStatus {
//     status: String
// }


