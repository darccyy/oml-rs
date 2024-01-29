fn main() {
    let file = include_str!("../example");
    let node = oml::from_str(file).expect("Failed to parse");
    println!("{:#?}", node);

    let table: Table = node.try_into().expect("Failed to convert to table");
    println!("{:#?}", table);
}

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Person>,
}
#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
    pub place_of_birth: String,
}

impl TryFrom<oml::Node> for Table {
    type Error = ();
    fn try_from(node: oml::Node) -> Result<Self, Self::Error> {
        let nodes = node.as_list().ok_or(())?;
        let mut rows = Vec::new();
        let mut nodes = nodes.into_iter();
        nodes.next().ok_or(())?.as_list().ok_or(())?;
        for node in nodes {
            let row: Person = node.try_into()?;
            rows.push(row);
        }
        Ok(Self { rows })
    }
}

impl TryFrom<oml::Node> for Person {
    type Error = ();
    fn try_from(node: oml::Node) -> Result<Self, Self::Error> {
        let nodes = node.as_list().ok_or(())?;
        let mut nodes = nodes.into_iter();
        let first_name = nodes.next().ok_or(())?.as_item().ok_or(())?;
        let last_name = nodes.next().ok_or(())?.as_item().ok_or(())?;
        let age = nodes
            .next()
            .ok_or(())?
            .as_item()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let place_of_birth = nodes.next().ok_or(())?.as_item().ok_or(())?;
        if nodes.next().is_some() {
            return Err(());
        }
        Ok(Self {
            first_name,
            last_name,
            age,
            place_of_birth,
        })
    }
}
