#![allow(unused)]
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub enum DataProtocol {
    UNDEFINED,
    JDBC,
    ODBC,
    S3,
    LOOKER,
}

#[derive(Debug, Clone)]
pub enum JobStatus {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub job_id: String,
    pub proto: DataProtocol,
    pub user_id: String,
    pub urgency: bool,
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct JobResult {
    pub result_id: String,
    pub job_id: String,
    pub status: JobStatus,
    pub dataset: String,
}

#[derive(Debug, Clone, Default)]
pub struct JobDB {
    queues: HashMap<DataProtocol, VecDeque<String>>,
    jobs: HashMap<String, Job>,
    results: HashMap<String, JobResult>,
}

pub type KJobDB = Arc<Mutex<JobDB>>;

impl JobDB {
    pub fn insert_job(&mut self, job: Job) {
        if let Some(queue) = self.queues.get_mut(&job.proto) {
            if !job.urgency {
                queue.push_back(job.job_id.clone());
            } else {
                queue.push_front(job.job_id.clone());
            }
            self.jobs.insert(job.job_id.clone(), job);
        } else {
            {
                log::warn!("No queue found for {:?}", job.proto);
            }
        }
    }
    pub fn pop_job(&mut self, proto: DataProtocol) -> Option<Job> {
        if let Some(queue) = self.queues.get_mut(&proto) {
            if let Some(id) = queue.pop_front() {
                let job_ref = self.jobs.get(&id).unwrap();
                let job = (*job_ref).clone();
                self.jobs.remove(&id);
                log::info!("Returning: {:?}", job);
                return Some(job);
            }
            log::info!("No job to pop from {:?}", proto);
            return None;
        } else {
            return None;
        }
    }
    pub fn peek_job(&self, job_id: String) -> Option<Job> {
        None
    }
    pub fn get_job(&self, job_id: String) -> Option<&Job> {
        self.jobs.get(&job_id)
    }
    pub fn insert_result(&mut self, result: JobResult) {
        self.results.insert(result.job_id.clone(), result);
    }
    pub fn get_result(&self, job_id: String) -> Option<&JobResult> {
        self.results.get(&job_id)
    }
    pub fn new() -> JobDB {
        let mut priority_queues = HashMap::new();
        priority_queues.insert(DataProtocol::JDBC, VecDeque::<String>::new());
        priority_queues.insert(DataProtocol::ODBC, VecDeque::<String>::new());
        priority_queues.insert(DataProtocol::S3, VecDeque::<String>::new());
        priority_queues.insert(DataProtocol::LOOKER, VecDeque::<String>::new());

        let job_list = HashMap::<String, Job>::new();
        let result_list = HashMap::<String, JobResult>::new();
        JobDB {
            queues: priority_queues,
            jobs: job_list,
            results: result_list,
        }
    }
}

pub fn build_sync_db() -> KJobDB {
    Arc::new(Mutex::new(JobDB::new()))
}
