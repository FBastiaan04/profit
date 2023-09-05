use rand::Rng;
use rand_distr::{Normal, Distribution};

#[derive(Clone,Copy)]
struct Job {
    start_time: u8,
    end_time: u8,
    profit: u32,
}

impl Job {
    fn new(start_time: u8, end_time: u8, profit: u32) -> Self {
        return Self {start_time, end_time, profit}
    }
}

struct Node {
    job: Job,
    children: Vec<Node>,
}

impl Node {
    fn new(job: Job, children: Vec<Node>) -> Self {
        Self {job, children}
    }
    fn add(mut self, child: Node) {
        self.children.push(child);
    }
}

fn gen_random_job(normal: Normal<u32>, mut rng: rand::rngs::ThreadRng) -> Job {
    let start_time = rng.gen();
    let end_time = rng.gen();
    let profit = normal
    return Job::new(start_time, end_time, profit)
}

fn next_branch(previous_end_time: u8, jobs: &Vec<Job>) -> Vec<Node> {
    // first, remove all jobs starting before the previous end time
    let jobs_left: Vec<Job> = jobs
        .iter()
        .filter(|&job| job.start_time < previous_end_time)
        .copied()
        .collect();
    
    // second, return nothing if there are no jobs after the last end time
    if jobs_left.len() == 0 {
        return vec![];
    }

    // third, find earliest end time. The next job must start before this time
    let earliest_end_time = jobs_left.iter().min_by(|a, b| a.end_time.cmp(&b.end_time)).unwrap().end_time;
    

    jobs_left
        .iter()
        .filter(|&j| j.start_time < earliest_end_time)
        .map(|&job| Node::new(job, next_branch(job.end_time, &jobs_left)))
        .collect()
}

fn main() {
    let normal = Normal::new(2.0, 3.0).unwrap();
    let mut rng = rand::thread_rng();
    let jobs = (0..100).into_iter().map(|_| Job::new(0, 0, 0));
}
