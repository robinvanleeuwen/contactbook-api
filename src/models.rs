use std::fmt;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use super::schema::contact;



#[derive(Queryable)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub phone1: Option<String>,
    pub phone2: Option<String>,
    pub phone3: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
}

impl Serialize for Contact {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let mut state = serializer.serialize_struct(
            "Contact",
            6
        )?;

        state.serialize_field("name", &self.name)?;
        state.serialize_field("phone1", &self.phone1)?;
        state.serialize_field("phone2", &self.phone2)?;
        state.serialize_field("phone3", &self.phone3)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()

    }

}

impl fmt::Display for Contact {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let phone1 = match &self.phone1 {
            Some(x) => x,
            None => "-",
        };

        let phone2 = match &self.phone2 {
            Some(x) => x,
            None => "-",
        };

        let phone3 = match &self.phone3 {
            Some(x) => x,
            None => "-",
        };

        let email = match &self.email {
            Some(x) => x,
            None => "-",
        };

        let comment = match &self.comment {
            Some(x) => x,
            None => "-",
        };

        let data = format!(
            "---\n\
            Name: {}\n\
            ---\n\
            Phone 1: {}\n\
            Phone 2: {}\n\
            Phone 3: {}\n\
            E-mail : {}\n\
            Comment: {}\n\
            ---
            ", self.name,
            phone1,
            phone2,
            phone3,
            email,
            comment
        );

        write!(f, "{}",data)


    }
}

#[derive(Insertable)]
#[table_name="contact"]
pub struct CreateContact {
    pub name: String,
    pub phone1: String,
    pub phone2: String,
    pub phone3: String,
    pub email: String,
    pub comment: String,
}