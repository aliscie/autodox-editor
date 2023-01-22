use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ElementTree {
    pub id: Id,
    pub elements: Tree<Id, EditorElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorElementCreate {
    pub id: Id,
    pub text: String,
    pub attrs: HashMap<String, String>,
    pub tree_id: Id,
    pub parent_id: Id,
    pub children: Option<Vec<Id>>,
    // represents the element after which the current element should be pushed
    pub prev_element_id: Option<Id>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorElementUpdate {
    pub id: Id,
    pub text: Option<String>,
    pub attrs: Option<HashMap<String, String>>,
    pub parent: Option<Id>,
    pub children: Option<IndexSet<Id>>,
}