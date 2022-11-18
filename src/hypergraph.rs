#[derive(Debug, Clone)]
pub struct Relation {
    pub related_nodes: Vec<NodeId>,
}

impl Relation {
    pub fn to_collection_form(&self) -> String {
        format!("{{{}}}", self.related_nodes.join(", "))
    }

    pub fn from_collection_form(collection_form: String) -> Self {
        if !collection_form.starts_with("{") || !collection_form.ends_with("}") {
            panic!("Invalid collection form {}", collection_form);
        } else {
            let related_nodes: Vec<NodeId> = collection_form
                .trim_start_matches("{")
                .trim_end_matches("}")
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            let relation = Relation { related_nodes };
            relation
        }
    }
}

type NodeId = String; // TODO maybe change?

#[derive(Debug, Clone)]
pub struct Hypergraph {
    pub relations: Vec<Relation>,
    pub nodes: Vec<NodeId>,
}

impl Hypergraph {
    pub fn to_collection_form(&self) -> String {
        let formatted_relations: Vec<String> = self
            .relations
            .iter()
            .map(|r| r.to_collection_form())
            .collect();
        return format!("{{ {} }}", formatted_relations.join(", "));
    }

    pub fn from_collection_form(collection_form: String) -> Self {
        if !collection_form.starts_with("{") || !collection_form.ends_with("}") {
            panic!("Invalid collection form {}", collection_form);
        } else {
            let relations_output: Vec<Relation> = get_relations_vec_str(
                &collection_form
                    .trim_start_matches("{")
                    .trim_end_matches("}")
                    .to_string(),
            )
            .iter()
            .map(|s| Relation::from_collection_form(s.to_owned() + "}"))
            .collect();
            let mut nodes: Vec<NodeId> = relations_output
                .clone()
                .into_iter()
                .map(|relation| relation.related_nodes)
                .flatten()
                .collect();
            nodes.sort();
            nodes.dedup();
            Hypergraph {
                relations: relations_output,
                nodes,
            }
        }
    }
}

fn get_relations_vec_str(relations_str: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut skipping = false;
    for c in relations_str.chars() {
        if c == '{' {
            result.push("{".to_string());
            skipping = false;
        } else if !skipping {
            if result.len() > 0 {
                result.last_mut().unwrap().push(c);
            }

            if c == '}' {
                skipping = true;
            }
        }
    }
    return result;
}

struct Rule {
    matches: Vec<Relation>,
    produces: Vec<Relation>,
}

impl Rule {
    fn eval(&self, initial: Hypergraph) -> Hypergraph {
        return initial;
    }
}
