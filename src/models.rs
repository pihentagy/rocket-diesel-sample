pub struct WorkloadWithoutId {
    pub repo_id: String,
    pub repo_version: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Workload {
    pub id: String,
    pub repo_id: String,
    pub repo_version: String,
}

#[derive(Deserialize)]
pub struct NewWorkload {
    pub id: Option<String>,
    pub repo_id: String,
    pub repo_version: String,
}
